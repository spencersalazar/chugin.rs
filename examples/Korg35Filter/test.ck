BlitSaw osc => Korg35 filter => dac;

100 => filter.freq;
0.5 => filter.K;
20 => osc.harmonics;

now + 5::second => time later;
while (now < later) {
    0.1*Math.pow(2, Math.sin(2*pi*(now/second)*1)) => float lfo;
    1500 + 1400*Math.sin(2*pi*(now/second)*lfo) => filter.freq;
    20::ms => now;
}
