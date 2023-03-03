import './css/App.scss';
import { CLK } from './clk';
import { ModuleTemplate } from './Modules';
import { Flags, FlagsType } from './alu';
import { CW } from './cw';
import { LCD, LCDState } from './lcd';

// TODO ideally we would want to move these global setters somewhere else
declare global {
    function set_cw(cw: number): void
    function set_flags(flags: FlagsType): void
    function set_lcd(display: LCDState): void
    function halt(): void
}

export function set_cw() {
    console.log("not set yet")
}
export function set_flags() {
    console.log("not set yet")
}
export function set_lcd() {
    console.log("not set yet")
}
export function halt() {
    console.log("not set yet");
}

const App = () => {
    return (
        <div className="App">
            <header className="App-header">
                <a style={{ fontWeight: 600 }} href="https://gitlab.com/MaksRawski/cbt/">CBT</a>
                <a href="https://gitlab.com/MaksRawski/cbt-emulator/"> emulator</a>
            </header>
            <div className="modules">
                <div className="module-column">
                    <CLK />
                    <div className="memory">
                        <ModuleTemplate name="MEMORY" />
                        <ModuleTemplate name="MAR" description="MEMORY ADDRESS REGISTER" />
                    </div>
                    <ModuleTemplate name="IR" description="INSTRUCTION REGISTER" />
                    <ModuleTemplate name="µT" id="utime" description="MICRO TIME/STEP COUNTER" />
                </div>
                <div className="BUS module-column">
                    <ModuleTemplate name="BUS" />
                </div>
                <div className="module-column">
                    <ModuleTemplate name="PC" description="PROGRAM COUNTER" />
                    <div className="row">
                        <ModuleTemplate name="RA" description="REGISTER A" />
                        <ModuleTemplate name="RB" description="REGISTER B" />
                    </div>
                    <div className="row">
                        <ModuleTemplate name="ALU" description="ARITHMETIC LOGIC UNIT" />
                        <Flags />
                    </div>
                    <div className="row">
                        <ModuleTemplate name="RC" description="REGISTER C" />
                        <ModuleTemplate name="RD" description="REGISTER D" />
                    </div>
                    <ModuleTemplate name="SP" description="STACK POINTER" />
                    <LCD />
                    <CW />
                </div>
            </div>
            <div className="footer"></div>
        </div >
    );
}


export default App;
