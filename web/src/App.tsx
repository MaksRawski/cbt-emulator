import React from 'react';
import './css/App.scss';
import { CLK } from './Modules';


function App() {
    return (
        <div className="App">
            <header className="App-header">
                CBT emulator
            </header>
            <div className="Modules">
                <div className="Left-side">
                    <CLK />
                    <CPUModule name="RAM" />
                    <CPUModule name="MAR" />
                    <CPUModule name="IR" />
                    <CPUModule name="µT" />
                </div>
                <div className="Right-side">
                    <CPUModule name="PC" />
                    <div className="row">
                        <CPUModule name="A" />
                        <CPUModule name="C" />
                    </div>
                    <CPUModule name="ALU" />
                    <div className="row">
                        <CPUModule name="B" />
                        <CPUModule name="D" />
                    </div>
                    <CPUModule name="FLAGS" />
                    <CPUModule name="SP" />
                    <CPUModule name="CW" />
                </div>
            </div>
            <div className="footer"></div>
        </div>
    );
}

interface ModuleData {
    name: string
    asdf?: number;

}
class CPUModule extends React.Component<ModuleData> {
    render() {
        return <p>Module: {this.props.name}</p>
    }
}


export default App;
