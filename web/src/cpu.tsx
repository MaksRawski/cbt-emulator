import init, { Cpu, setup_logging } from 'cbt_emulator';
export let cpu: Cpu;

init().then(() => {
    setup_logging();
    cpu = Cpu.new();
    cpu.load_program(hello_world);
    console.log("rom: ", cpu.view_rom());
});

export const hello_world = new Uint8Array();
