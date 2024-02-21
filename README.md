# sweep.wav FIR MATLAB Plot
## Gain = 0.5, Delay = 1.0 seconds, Channels=2
<img width="1150" alt="Screenshot 2024-02-20 at 6 53 37 PM" src="https://github.com/emurray2/ase-2024/assets/15041342/a8ad4bb0-ffeb-4558-8056-e2400d81c3f6">

# sweep.wav FIR MATLAB Code
## Gain = 0.5, Delay = 1.0 seconds, Channels=2
<details>
  <summary> FIR MATLAB Code</summary>

  ```
  [x,Fs] = audioread('sweep.wav');
  [rust,Fs_rust] = audioread('sweepdata_rust.wav');
  g=0.5;
  x_size=size(x);
  channels=x_size(2);
  Delayline=zeros(Fs*1.0,channels);
  y = zeros(length(x),channels);
  rust = rust(1:length(y),:);
  tt=linspace(0,length(y)/Fs,length(y));
  for n=1:length(x)
      for channel=1:channels
          y(n,channel)=x(n,channel)+g*Delayline(length(Delayline),channel);
          Delayline(:,channel)=[x(n,channel);Delayline(1:length(Delayline)-1,channel)];
      end
  end
  diff = rust - y;
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

# sweep.wav IIR MATLAB Plot
## Gain = 0.5, Delay = 1.0 seconds, Channels=2
<img width="1150" alt="Screenshot 2024-02-20 at 7 16 36 PM" src="https://github.com/emurray2/ase-2024/assets/15041342/f156685f-1246-4558-89fb-86168548df73">

# sweep.wav IIR MATLAB Code
## Gain = 0.5, Delay = 1.0 seconds, Channels=2
<details>
  <summary> IIR MATLAB Code</summary>

  ```
  [x,Fs] = audioread('sweep.wav');
  [rust,Fs_rust] = audioread('sweepdata_rust.wav');
  g=0.5;
  x_size=size(x);
  channels=x_size(2);
  Delayline=zeros(Fs*1.0,channels);
  y = zeros(length(x),channels);
  rust = rust(1:length(y),:);
  tt=linspace(0,length(y)/Fs,length(y));
  for n=1:length(x)
      for channel=1:channels
          y(n,channel)=x(n,channel)+g*Delayline(length(Delayline),channel);
          Delayline(:,channel)=[y(n,channel);Delayline(1:length(Delayline)-1,channel)];
      end
  end
  diff = rust - y;
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

