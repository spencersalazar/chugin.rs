RustBlit osc1 => Gain g => dac;
Blit osc2 => g;
2 => g.op; // difference

0 => Std.srand;

now + 5::second => time later;
while (now < later) {
    Std.rand2(48, 72) => Std.mtof => float f;
    f => osc1.freq;
    f => osc2.freq;
    0.25::second => now;
}
