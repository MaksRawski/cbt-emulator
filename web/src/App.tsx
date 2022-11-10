import './css/App.scss';
import { CLK } from './clk';
import { ModuleTemplate } from './Modules';
import { Flags } from './alu';
import { CW } from './cw';

// TODO ideally we would want to move these global setters somewhere else
declare global{
    function set_cw(cw: number): void
    function set_flags(flags: {c: boolean, h: boolean, o: boolean, z: boolean}): void
}

export function set_cw(cw: number){
    console.log("not set yet")
}
export function set_flags(cw: number){
    console.log("not set yet")
}

global.set_cw = set_cw;

const App = () => {
    return (
        <div className="App">
            <header className="App-header">
                <a style={{fontWeight: 600}} href="https://gitlab.com/MaksRawski/cbt/">CBT</a>
                <a href="https://gitlab.com/MaksRawski/cbt-emulator/"> emulator</a>
            </header>
            <div className="modules">
                <div className="module-column">
                    <CLK />
                    <div className="memory">
                        <ModuleTemplate name="RAM" />
                        <ModuleTemplate name="MAR" />
                    </div>
                    <ModuleTemplate name="IR" />
                    <ModuleTemplate name="µT" id="utime" />
                </div>
                <div className="BUS module-column">
                    <ModuleTemplate name="BUS" />
                </div>
                <div className="module-column">
                    <ModuleTemplate name="PC" />
                    <div className="row">
                        <ModuleTemplate name="RA" />
                        <ModuleTemplate name="RB" />
                    </div>
                    <div className="row">
                        <ModuleTemplate name="ALU" />
                        <Flags />
                    </div>
                    <div className="row">
                        <ModuleTemplate name="RC" />
                        <ModuleTemplate name="RD" />
                    </div>
                    <ModuleTemplate name="SP" />
                    <LCD />
                    <CW />
                </div>
            </div>
            <div className="footer"></div>
        </div >
    );
}


export default App;
