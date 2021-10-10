BlitSaw osc => Korg35 filter => dac;

100 => filter.freq;
1.8 => filter.K;
50 => osc.harmonics;

now + 5::second => time later;
while (now < later) {
    0.25 => float lfo;
    1500 + 1400*Math.sin(2*pi*(now/second)*lfo) => filter.freq;
    20::ms => now;
}
