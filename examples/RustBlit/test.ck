RustBlit osc => dac;

0 => Std.srand;

now + 5::second => time later;
while (now < later) {
    Std.rand2(48, 72) => Std.mtof => osc.freq;
    0.25::second => now;
}
