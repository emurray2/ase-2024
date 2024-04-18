
use crate::ring_buffer::RingBuffer;
use realfft::RealFftPlanner;
use rustfft::{self, FftPlanner};
use rustfft::num_complex::{Complex, ComplexFloat};

pub struct FastConvolver {
    impulse_response : Vec<f32>,
    mode : ConvolutionMode,
    conv_buffer : RingBuffer<f32>,
    impulse_response_block : Vec<Vec<f32>>,
    impulse_response_dft_block : Vec<Vec<Complex<f32>>>,
    block_size : usize, 
    output_buffer : RingBuffer<f32>,
}

#[derive(Debug, Clone, Copy)]
pub enum ConvolutionMode {
    TimeDomain,
    FrequencyDomain { block_size: usize }
}

impl FastConvolver {
    pub fn new(impulse_response: &[f32], mode: ConvolutionMode) -> Self {
        let mut conv_block_size : usize = 0; 
        let mut impulse_response_block : Vec<Vec<f32>> = vec![];
        let mut impulse_response_dft_block : Vec<Vec<Complex<f32>>> = vec![];
        let mut buffer_size : usize = 0;
        let mut buffer_size_output : usize = 0;
        match mode
        {
            ConvolutionMode::FrequencyDomain{block_size} => 
            {
                conv_block_size = block_size;
                FastConvolver::get_impulse_block(impulse_response, conv_block_size, 
                    &mut impulse_response_block, &mut impulse_response_dft_block); 
                let block_count = impulse_response_dft_block.len();
                buffer_size = conv_block_size * 2 + 5; // +5 additional space to avoid bugs
                buffer_size_output = block_count * conv_block_size;

                buffer_size_output += 5;
            },
            ConvolutionMode::TimeDomain => 
            {
                buffer_size = impulse_response.len() * 2;
            },
        }

        let mut ring_buffer_output : RingBuffer<f32> = RingBuffer::new(buffer_size_output);
        let block_count = impulse_response_dft_block.len();
        for _ in 0..(block_count * conv_block_size)
        {
            ring_buffer_output.push(0.0);
        }

        FastConvolver{
            impulse_response: impulse_response.to_vec(), 
            mode,
            conv_buffer : RingBuffer::new(buffer_size),
            impulse_response_block,
            impulse_response_dft_block,
            block_size : conv_block_size,
            output_buffer : ring_buffer_output,
        }
    
    }
    pub fn get_impulse_block(impulse_response: &[f32], block_size : usize,
                        impulse_response_block : &mut Vec<Vec<f32>>,
                        impulse_response_dft_block : &mut Vec<Vec<Complex<f32>>>)
    {
        let zeropad = vec![0.0; block_size];
        impulse_response.chunks_exact(block_size).for_each(|chunk| {
            impulse_response_block.push(chunk.to_vec());
        });
        if impulse_response.len() % block_size > 0 {
            let remaining = impulse_response.chunks_exact(block_size).remainder().to_vec();
            let remainingzeros = vec![0.0; block_size-remaining.len()];
            let last_block = [remaining, remainingzeros].concat();
            impulse_response_block.push(last_block)
        }
        let mut real_planner = RealFftPlanner::<f32>::new();
        for block in impulse_response_block.iter_mut()
        {
            // add zeros to the end of each block
            block.extend_from_slice(&zeropad);

            // do fft
            let r2c = real_planner.plan_fft_forward(block_size * 2);
            let mut out_data = vec![Complex::new(0.0 as f32,0.0 as f32); block_size + 1];
            r2c.process(block, &mut out_data).unwrap();
            for cur_index in 1..block_size
            {
                out_data.push(out_data[block_size - cur_index].conj())
            }
            impulse_response_dft_block.push(out_data);
        }

    }

    pub fn reset(&mut self) {
        self.conv_buffer.reset();
    }

    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        match self.mode
        {
            ConvolutionMode::FrequencyDomain{block_size} => 
            {
                panic!("Error: please use frequency domain process!");
            },
            ConvolutionMode::TimeDomain => 
            {
            },
        }

        if input.len() != output.len()
        {
            panic!("error: mismatched input and output size!");
        }

        for (output_index, output_sample) in output.iter_mut().enumerate()
        {
            self.conv_buffer.push(input[output_index]);
            *output_sample = self.calculate_conv_sum();
        }
    }
    pub fn process_freq(&mut self, input: &[f32]) -> Vec<f32>{
        // In order to process the delays with fastconv, this function should return a value
        // If there were enough data in the buffer (more than two blocks),
        // then the return value is a non-empty vector, which is the results of the fast conv
        // If there were not enough data in the buffer,
        // then the return value is an empty vector. 
        let mut final_result : Vec<f32> = vec![];
        match self.mode
        {
            ConvolutionMode::FrequencyDomain{block_size} => 
            {
            },
            ConvolutionMode::TimeDomain => 
            {
                panic!("Error: please use time domain process!");
            },
        }
        let mut fft_planner = FftPlanner::<f32>::new();
        let r2c = fft_planner.plan_fft_forward(self.block_size * 2);
        let fft_planner_inv = FftPlanner::<f32>::new();
        let r2c_inv = fft_planner.plan_fft_inverse(self.block_size * 2);
        for input_sample in input.iter() {
            self.conv_buffer.push(*input_sample);
            if self.conv_buffer.len()/self.block_size == 2
            {
                let mut complex_input : Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); self.block_size * 2];
                for buffer_index in 0..self.block_size
                {
                    let re_part = self.conv_buffer.get(buffer_index);
                    let im_part = self.conv_buffer.get(self.block_size + buffer_index);
                    let test_val = Complex::new(re_part, im_part);
                    complex_input[buffer_index] = test_val;
                }
                r2c.process(&mut complex_input);
                // now we have: 
                // complex_input - fft of the input
                let mut freq_domain_multiplication : Vec<Vec<Complex<f32>>> = vec![];
                for (cur_block, block) in self.impulse_response_dft_block.iter().enumerate()
                {
                    freq_domain_multiplication.push(vec![]);
                    for cur_number in 0..(self.block_size * 2)
                    {
                        freq_domain_multiplication[cur_block].push(complex_input[cur_number] * 
                                        block[cur_number]) 
                    }
                    // calculate inverse fft 
                    r2c_inv.process(&mut freq_domain_multiplication[cur_block]);
                    // divide by 1/blocksize for normalization
                    for cur_number in 0..(self.block_size*2)
                    {
                        freq_domain_multiplication[cur_block][cur_number] /= 10.0;
                    }
                }
                let block_count = freq_domain_multiplication.len();
                let mut output_sum : Vec<f32> = vec![0.0; 
                        (block_count + 2) * self.block_size];
                for cur_block in 0..(freq_domain_multiplication.len())
                {
                    let start_pos = cur_block * self.block_size;
                    for cur_multiplication_res in 0..(self.block_size * 2)
                    {
                        output_sum[start_pos + cur_multiplication_res] += 
                                freq_domain_multiplication[cur_block][cur_multiplication_res].re;
                    }
                }
                for cur_block in 0..(freq_domain_multiplication.len())
                {
                    let start_pos = (cur_block + 1) * self.block_size;
                    for cur_multiplication_res in 0..(self.block_size * 2)
                    {
                        output_sum[start_pos + cur_multiplication_res] += 
                                freq_domain_multiplication[cur_block][cur_multiplication_res].im;
                    }
                }
                // for the first two blocks of data, pop and add as the final result
                let mut read_index : usize = 0;
                for cur_index in 0..(self.block_size*2)
                {
                    let cur_buffer_value = self.output_buffer.pop();
                    let cur_output = output_sum[cur_index] + cur_buffer_value;
                    final_result.push(cur_output);
                    read_index += 1;
                }
                // for the middle blocks, add to the buffer but not pop
                // if there are less than 2 blocks then this operation is skipped. 
                for cur_index in 0..(block_count + 2 - 4) * self.block_size
                {
                    let cur_buffer_value = self.output_buffer.get(cur_index);
                    let cur_sum = output_sum[read_index];
                    let new_sum = cur_sum + cur_buffer_value;
                    self.output_buffer.set(cur_index, new_sum);
                    read_index += 1;
                }
                // for the leftover, push to the buffer 
                for cur_index in read_index..(output_sum.len())
                {
                    self.output_buffer.push(output_sum[cur_index]);
                }

            }
        }
        final_result
    }

    pub fn get_conv_length(&self) -> usize {
        if self.conv_buffer.len() < self.impulse_response.len()
        {
            self.conv_buffer.len()
        }
        else 
        {
            self.impulse_response.len()
        }
    }

    pub fn calculate_conv_sum(&self) -> f32 {
        let conv_length = self.get_conv_length();
        let mut conv_sum : f32 = 0.0;
        for conv_index in 0..conv_length
        {
            let buffer_index = self.conv_buffer.len() - conv_index - 1;
            let cur_data = self.conv_buffer.get(buffer_index);
            let cur_kernel = self.impulse_response[conv_index];
            conv_sum += cur_data * cur_kernel;
        }
        conv_sum
    }

    pub fn flush(&mut self, output: &mut [f32]) {
        for (output_index, output_sample) in output.iter_mut().enumerate()
        {
            self.conv_buffer.push(0.0);
            *output_sample = self.calculate_conv_sum();
        }
    }
    pub fn flush_freq(&mut self)->Vec<Vec<f32>>
    {
        let mut remain_and_flush : Vec<Vec<f32>> = vec![];
        let mut remain_output : Vec<f32> = vec![];
        let mut flush_output : Vec<f32> = vec![];
        if self.conv_buffer.len() == 0
        {
            remain_and_flush.push(remain_output);
        }
        else  
        {
            // first process remaining data 
            let true_data_length = self.conv_buffer.len();
            let remaining_data_length = self.block_size * 2 - self.conv_buffer.len();
            let extra_zero = vec![0.0; remaining_data_length];
            let zero_output = self.process_freq(&extra_zero);
            for cur_remain_index in 0..true_data_length
            {
                remain_output.push(zero_output[cur_remain_index]);
            }
            for cur_remain_index in true_data_length..zero_output.len()
            {
                flush_output.push(zero_output[cur_remain_index]);
            }
            remain_and_flush.push(remain_output);
        }

        // first the remaining output, then the flush output
        let extra_zero = vec![0.0; 
            self.impulse_response_block.len() * (self.block_size + 2)]; // put two more blocks to avoid bugs
        let zero_output = self.process_freq(&extra_zero);
        for zero_output_index in 0..self.impulse_response.len() - 1
        {
            if flush_output.len() >= self.impulse_response.len() - 1
            {
                break;
            }
            flush_output.push(zero_output[zero_output_index]);
        }
        remain_and_flush.push(flush_output);
        
        remain_and_flush
        

    }

    // TODO: feel free to define other functions for your own use
    pub fn get_tail_size(&self) -> usize {
        self.impulse_response.len() - 1
    }
}

// TODO: feel free to define other types (here or in other modules) for your own use

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use rand::prelude::*;
    use rustfft::num_traits::abs;
    use super::*;

    #[test]
    fn test_identity()
    {
        let mut random_ir = vec![0.0; 51];
        let mut input = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let mut rng = rand::thread_rng();
        for ir_index in 0..51 
        {
            random_ir[ir_index] = rng.gen();
        }
        let mut convolver = FastConvolver::new(&random_ir, ConvolutionMode::TimeDomain);
        let mut output = vec![0.0; 10];
        convolver.process(&input,  &mut output);
        let mut target_output = vec![0.0; 10];
        for data_index in 0..7
        {
            target_output[data_index + 3] = random_ir[data_index];
        }
        for data_index in 0..10
        {
            assert_eq!(output[data_index], target_output[data_index]);
        }
    }
    #[test]
    fn test_flush()
    {
        let mut random_ir = vec![0.0; 51];
        let mut input = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let mut rng = rand::thread_rng();
        for ir_index in 0..51 
        {
            random_ir[ir_index] = rng.gen();
        }
        let mut convolver = FastConvolver::new(&random_ir, ConvolutionMode::TimeDomain);
        let mut output = vec![0.0; 10];
        let mut tail = vec![0.0; convolver.get_tail_size()];
        convolver.process(&input,  &mut output);
        convolver.flush(&mut tail);

        let mut target_output = vec![0.0; tail.len()];
        for data_index in 7..51
        {
            target_output[data_index - 7] = random_ir[data_index];
        }
        for data_index in 0..50
        {
            assert_eq!(tail[data_index], target_output[data_index]);
        }
    }
    #[test]
    fn test_block()
    {
        let mut random_ir = vec![0.0; 51];
        let mut input = vec![0.0; 10000];
        input[3] = 1.0;
        let mut rng = rand::thread_rng();
        for ir_index in 0..51 
        {
            random_ir[ir_index] = rng.gen();
        }
        let mut convolver = FastConvolver::new(&random_ir, ConvolutionMode::TimeDomain);
        let mut output = vec![0.0; 10000];

        convolver.process(&input[0..1],  &mut output[0..1]);
        convolver.process(&input[1..14],  &mut output[1..14]);
        convolver.process(&input[14..1037],  &mut output[14..1037]);
        convolver.process(&input[1037..3085],  &mut output[1037..3085]);
        convolver.process(&input[3085..3086],  &mut output[3085..3086]);
        convolver.process(&input[3086..3103],  &mut output[3086..3103]);
        convolver.process(&input[3103..8103],  &mut output[3103..8103]);
        convolver.process(&input[8103..10000],  &mut output[8103..10000]);
        let mut target_output = vec![0.0; 10000];
        for data_index in 0..51
        {
            target_output[data_index + 3] = random_ir[data_index];
        }
        for data_index in 0..10000
        {
            assert_eq!(output[data_index], target_output[data_index]);
        }
    }
    #[test]
    fn test_fast_conv()
    {
        let mut real_planner = RealFftPlanner::<f32>::new();
        let r2c = real_planner.plan_fft_forward(10);
        
        let mut random_ir = vec![0.0; 51];
        let mut input = vec![0.0; 10000];
        input[0] = 0.0;
        input[1] = 0.1;
        input[2] = 0.2;
        input[3] = 0.3;
        input[4] = 0.4;
        input[5] = 0.5;
        input[6] = 0.6;
        input[7] = 0.7;
        input[8] = 0.8;
        input[9] = 0.9;
        input[10] = 1.0;
        input[11] = 1.1;


        let mut rng = rand::thread_rng();
        for ir_index in 0..51 
        {
            random_ir[ir_index] = rng.gen();
            println!("{}", random_ir[ir_index]);
        }
        println!("*********************");
        let mut convolver = FastConvolver::new(&random_ir,
                         ConvolutionMode::FrequencyDomain { block_size: 5 });
        
        
        let result = convolver.process_freq(&input[0..100]);  
        for cur_result in result
        {
            println!("{}", cur_result);
        }      
    }
    #[test]
    fn test_fast_conv_to_normal_conv()
    {
        let mut random_ir = vec![0.0; 51];
        let mut input = vec![0.0; 10000];
        
        let mut rng = rand::thread_rng();
        for ir_index in 0..51 
        {
            random_ir[ir_index] = rng.gen();
        }
        for input_index in 0..10000
        {
            input[input_index] = rng.gen();
        }

        let mut convolver = FastConvolver::new(&random_ir, ConvolutionMode::TimeDomain);
        let mut output = vec![0.0; 10000];

        convolver.process(&input[0..10000],  &mut output[0..10000]);

        let mut convolver = FastConvolver::new(&random_ir,
            ConvolutionMode::FrequencyDomain { block_size: 5 });


        let result = convolver.process_freq(&input[0..10000]);  
        for cur_index in 0..result.len()
        {
            //assert_eq!(result[cur_index], output[cur_index]);
            assert!(abs(result[cur_index] - output[cur_index]) < 0.001);
        }
    }
    #[test]
    fn test_fast_conv_identity()
    {
        let mut random_ir = vec![0.0; 51];
        let mut input = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let mut rng = rand::thread_rng();
        for ir_index in 0..51 
        {
            random_ir[ir_index] = rng.gen();
        }
        let mut convolver = FastConvolver::new(&random_ir, 
                    ConvolutionMode::FrequencyDomain { block_size: 5 });

        let mut output = convolver.process_freq(&input);
        let mut target_output = vec![0.0; 10];
        for data_index in 0..7
        {
            target_output[data_index + 3] = random_ir[data_index];
        }
        for data_index in 0..10
        {
            assert!(abs(output[data_index]- target_output[data_index]) < 0.001);
        }
    }
    #[test]
    fn test_flush_freq()
    {
        let mut random_ir = vec![0.0; 51];
        let mut input = vec![0.0; 9994];
        
        let mut rng = rand::thread_rng();
        for ir_index in 0..51 
        {
            random_ir[ir_index] = rng.gen();
        }
        for input_index in 0..9994
        {
            input[input_index] = rng.gen();
        }

        let mut convolver = FastConvolver::new(&random_ir, ConvolutionMode::TimeDomain);
        let mut output = vec![0.0; 9994];
        let mut tail_time_domain = vec![0.0; 50];
        convolver.process(&input[0..9994],  &mut output[0..9994]);
        convolver.flush(&mut tail_time_domain);

        let mut convolver = FastConvolver::new(&random_ir,
            ConvolutionMode::FrequencyDomain { block_size: 5 });


        let mut result = convolver.process_freq(&input[0..9994]);  
        let flush_and_remain = convolver.flush_freq();
        let remain_result = &flush_and_remain[0];
        println!("{}", remain_result.len());
        for cur_remain in remain_result
        {
            result.push(*cur_remain);
        }
        for cur_index in 0..result.len()
        {
            println!("{}", cur_index);
            //assert_eq!(result[cur_index], output[cur_index]);
            assert!(abs(result[cur_index] - output[cur_index]) < 0.001);
        }
        let flush_result = &flush_and_remain[1];
        for cur_index in 0..tail_time_domain.len()
        {
            println!("{} {}", flush_result[cur_index], tail_time_domain[cur_index]);
            //assert!(abs(flush_result[cur_index] - tail_time_domain[cur_index]) < 0.01);
        }
    }
    #[test]
    fn test_block_freq()
    {
        let mut random_ir = vec![0.0; 51];
        let mut input = vec![0.0; 10000];
        input[3] = 1.0;
        let mut rng = rand::thread_rng();
        for ir_index in 0..51 
        {
            random_ir[ir_index] = rng.gen();
        }
        let mut convolver = FastConvolver::new(&random_ir, 
                        ConvolutionMode::FrequencyDomain { block_size: 5 });
        let mut output : Vec<f32> = vec![];

        output.append(&mut convolver.process_freq(&input[0..1]));
        output.append(&mut convolver.process_freq(&input[1..14]));
        output.append(&mut convolver.process_freq(&input[14..1037]));
        output.append(&mut convolver.process_freq(&input[1037..3085]));
        output.append(&mut convolver.process_freq(&input[3085..3086]));
        output.append(&mut convolver.process_freq(&input[3086..3103]));
        output.append(&mut convolver.process_freq(&input[3103..8103]));
        output.append(&mut convolver.process_freq(&input[8103..10000]));
        let mut target_output = vec![0.0; 10000];
        for data_index in 0..51
        {
            target_output[data_index + 3] = random_ir[data_index];
        }
        for data_index in 0..10000
        {
            assert!(abs(output[data_index] - target_output[data_index]) < 0.001);
        }
    }
}
