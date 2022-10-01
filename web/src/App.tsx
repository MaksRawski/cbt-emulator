import './css/App.scss';
import { CLK } from './clk';
import { ModuleTemplate } from './Modules';
import { Flags } from './alu';

const App = () => {
    return (
        <div className="App">
            <header className="App-header">
                CBT emulator
            </header>
            <div className="Modules">
                <div className="Left-side">
                    <CLK />
                    <ModuleTemplate name="RAM" />
                    <ModuleTemplate name="MAR" />
                    <ModuleTemplate name="IR" />
                    <ModuleTemplate name="µTime" id="utime" />
                </div>
                <div className='BUS'>
                    <ModuleTemplate name="BUS" />
                </div>
                <div className="Right-side">
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
                    <ModuleTemplate name="CW" />
                </div>
            </div>
            <div className="footer"></div>
        </div>
    );
}


export default App;
