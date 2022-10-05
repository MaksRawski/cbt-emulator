import './css/App.scss';
import { CLK } from './clk';
import { ModuleTemplate } from './Modules';
import { Flags } from './alu';

const App = () => {
    return (
        <div className="App">
            <header className="App-header">
                <a target="_blank" rel="norefferer" href="https://gitlab.com/MaksRawski/cbt/">CBT</a> emulator
            </header>
            <div className="modules">
                <div className="left-side">
                    <CLK />
                    <ModuleTemplate name="RAM" />
                    <ModuleTemplate name="MAR" />
                    <ModuleTemplate name="IR" />
                    <ModuleTemplate name="µT" id="utime" />
                </div>
                <div className='BUS'>
                    <ModuleTemplate name="BUS" />
                </div>
                <div className="right-side">
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
                    <div className="CW">
                        <ModuleTemplate name="CW" />
                    </div>
                </div>
            </div>
            <div className="footer"></div>
        </div >
    );
}


export default App;
