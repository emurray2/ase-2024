# Test 1 - sweep.wav
## Samplerate = 44100.0, Mod Frequency = 10 Hertz, Delay Time: 0.5 seconds, Channels=2
## MATLAB Plot
<img width="1162" alt="Screenshot 2024-02-29 at 11 33 46 AM" src="https://github.com/emurray2/ase-2024/assets/15041342/57d902a5-bb2b-48fc-8949-b38614918ce1">

## MATLAB Code
<details>
  <summary> vibrato_modified.m</summary>

  ```
  [x,Fs] = audioread('sweep.wav');
  [rust,Fs_rust] = audioread('sweepdata_rust.wav');
  y = vibrato(x,44100,10,0.5,2);
  rust = rust(1:length(y),:);
  tt=linspace(0,length(y)/Fs,length(y));
  diff = y - rust;
  figure
  subplot(4,1,1)
  plot(tt, y(:,1))
  title('MATLAB')
  subplot(4,1,2)
  plot(tt, y(:,2))
  subplot(4,1,3)
  plot(tt, diff(:,1))
  ylim([-1 1])
  title('Difference')
  subplot(4,1,4)
  plot(tt, diff(:,2))
  ylim([-1 1])
  filename = 'sweepdata_matlab.wav';
  audiowrite(filename,y,Fs);
  ```
</details>
<details>
  <summary> vibrato.m</summary>

  ```
  function y=vibrato(x,SAMPLERATE,Modfreq,Width,num_channels)
  % Author: S. Disch
  %
  %--------------------------------------------------------------------------
  % This source code is provided without any warranties as published in 
  % DAFX book 2nd edition, copyright Wiley & Sons 2011, available at 
  % http://www.dafx.de. It may be used for educational purposes and not 
  % for commercial applications without further permission.
  %--------------------------------------------------------------------------
  
  ya_alt=0;
  Delay=Width; % basic delay of input sample in sec
  DELAY=round(Delay*SAMPLERATE); % basic delay in # samples
  WIDTH=round(Width*SAMPLERATE); % modulation width in # samples
  if WIDTH>DELAY 
    error('delay greater than basic delay !!!');
    return;
  end
  MODFREQ=Modfreq/SAMPLERATE; % modulation frequency in # samples
  LEN=length(x);        % # of samples in WAV-file
  L=2+DELAY+WIDTH*2;    % length of the entire delay  
  Delayline=zeros(L,num_channels); % memory allocation for delay
  y=zeros(size(x));     % memory allocation for output vector
  for channel = 1:num_channels
      for n=1:(LEN-1)
         M=MODFREQ;
         MOD=sin(M*2*pi*n);
         TAP=1+DELAY+WIDTH*MOD;
         i=floor(TAP);
         frac=TAP-i;
         Delayline=[x(n,:);Delayline(1:L-1, :)];
         %---Linear Interpolation-----------------------------
         y(n,channel)=Delayline(i+1, channel)*frac+Delayline(i, channel)*(1-frac);
         %---Allpass Interpolation------------------------------
         %y(n,1)=(Delayline(i+1)+(1-frac)*Delayline(i)-(1-frac)*ya_alt);  
         %ya_alt=ya(n,1);
         %---Spline Interpolation-------------------------------
         %y(n,1)=Delayline(i+1)*frac^3/6
         %....+Delayline(i)*((1+frac)^3-4*frac^3)/6
         %....+Delayline(i-1)*((2-frac)^3-4*(1-frac)^3)/6
         %....+Delayline(i-2)*(1-frac)^3/6; 
         %3rd-order Spline Interpolation
      end
  end
  end  
  ```
</details>

# Test 2 - drumloop.wav
## Samplerate = 44100.0, Mod Frequency = 1 Hertz, Delay Time: 0.01 seconds, Channels=1
## MATLAB Plot
<img width="1155" alt="Screenshot 2024-02-29 at 11 39 45 AM" src="https://github.com/emurray2/ase-2024/assets/15041342/95ed7165-1d01-4bd0-b0d9-762c377148ab">

## MATLAB Code
<details>
  <summary> vibrato_modified.m</summary>

  ```
  [x,Fs] = audioread('drumloop.wav');
  [rust,Fs_rust] = audioread('drumloopdata_rust.wav');
  y = vibrato(x,44100,1,0.01,1);
  y = y(1:length(rust),:);
  tt=linspace(0,length(y)/Fs,length(y));
  diff = y - rust;
  figure
  subplot(4,1,1)
  plot(tt, y(:,1))
  title('MATLAB')
  subplot(4,1,2)
  %plot(tt, y(:,2))
  subplot(4,1,3)
  plot(tt, diff(:,1))
  ylim([-1 1])
  title('Difference')
  subplot(4,1,4)
  %plot(tt, diff(:,2))
  ylim([-1 1])
  filename = 'drumloopdata_matlab.wav';
  audiowrite(filename,y,Fs);
  ```
</details>
<details>
  <summary> vibrato.m</summary>

  ```
  function y=vibrato(x,SAMPLERATE,Modfreq,Width,num_channels)
  % Author: S. Disch
  %
  %--------------------------------------------------------------------------
  % This source code is provided without any warranties as published in 
  % DAFX book 2nd edition, copyright Wiley & Sons 2011, available at 
  % http://www.dafx.de. It may be used for educational purposes and not 
  % for commercial applications without further permission.
  %--------------------------------------------------------------------------
  
  ya_alt=0;
  Delay=Width; % basic delay of input sample in sec
  DELAY=round(Delay*SAMPLERATE); % basic delay in # samples
  WIDTH=round(Width*SAMPLERATE); % modulation width in # samples
  if WIDTH>DELAY 
    error('delay greater than basic delay !!!');
    return;
  end
  MODFREQ=Modfreq/SAMPLERATE; % modulation frequency in # samples
  LEN=length(x);        % # of samples in WAV-file
  L=2+DELAY+WIDTH*2;    % length of the entire delay  
  Delayline=zeros(L,num_channels); % memory allocation for delay
  y=zeros(size(x));     % memory allocation for output vector
  for channel = 1:num_channels
      for n=1:(LEN-1)
         M=MODFREQ;
         MOD=sin(M*2*pi*n);
         TAP=1+DELAY+WIDTH*MOD;
         i=floor(TAP);
         frac=TAP-i;
         Delayline=[x(n,:);Delayline(1:L-1, :)];
         %---Linear Interpolation-----------------------------
         y(n,channel)=Delayline(i+1, channel)*frac+Delayline(i, channel)*(1-frac);
         %---Allpass Interpolation------------------------------
         %y(n,1)=(Delayline(i+1)+(1-frac)*Delayline(i)-(1-frac)*ya_alt);  
         %ya_alt=ya(n,1);
         %---Spline Interpolation-------------------------------
         %y(n,1)=Delayline(i+1)*frac^3/6
         %....+Delayline(i)*((1+frac)^3-4*frac^3)/6
         %....+Delayline(i-1)*((2-frac)^3-4*(1-frac)^3)/6
         %....+Delayline(i-2)*(1-frac)^3/6; 
         %3rd-order Spline Interpolation
      end
  end
  end  
  ```
</details>
