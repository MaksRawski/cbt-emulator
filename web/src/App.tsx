import React, { useState } from 'react';
import './css/App.scss';
import { CLK } from './Modules';

function App() {
    const [clk, setCLK] = useState(false);
    return (
        <div className="App">
            <header className="App-header">
                CBT emulator
            </header>
            <div className="Modules">
                <div className="Left-side">
                    <CLK clkState={{ v: clk, d: setCLK }} />
                    <CPUModule name="RAM" clk={clk} />
                    <CPUModule name="MAR" clk={clk} />
                    <CPUModule name="IR" clk={clk} />
                    <CPUModule name="µT" clk={clk} />
                </div>
                <div className="Right-side">
                    <CPUModule name="PC" clk={clk} />
                    <div className="row">
                        <CPUModule name="A" clk={clk} />
                        <CPUModule name="C" clk={clk} />
                    </div>
                    <CPUModule name="ALU" clk={clk} />
                    <div className="row">
                        <CPUModule name="B" clk={clk} />
                        <CPUModule name="D" clk={clk} />
                    </div>
                    <CPUModule name="FLAGS" clk={clk} />
                    <CPUModule name="SP" clk={clk} />
                    <CPUModule name="CW" clk={clk} />
                </div>
            </div>
            <div className="footer"></div>
        </div>
    );
}

interface ModuleData {
    name: string
    clk: boolean,

}
class CPUModule extends React.Component<ModuleData> {
    render() {
        return <p>Module: {this.props.name}</p>
    }
}


export default App;
