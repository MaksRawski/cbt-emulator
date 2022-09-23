import React, { useState } from 'react';
import './css/App.scss';
import { CLK } from './clk';
import { Register } from './register';
import { reactSetter, CPUModule } from './Modules';

declare global {
    var clk: boolean
    var setters: Map<String, reactSetter<any>>
}

const App = () => {
    var clkSetter: reactSetter<boolean>
    [global.clk, clkSetter] = useState(false);
    global.setters = new Map
    global.setters.set("clk", clkSetter);
    return (
        <div className="App">
            <header className="App-header">
                CBT emulator
            </header>
            <div className="Modules">
                <div className="Left-side">
                    <CLK />
                    {/*  wasm_name="RAM"*/}
                    <CPUModule />
                    {/*  wasm_name="MAR"*/}
                    <CPUModule />
                    {/*  wasm_name="IR"*/}
                    <CPUModule />
                    {/*  wasm_name="µT"*/}
                    <CPUModule />
                </div>
                <div className="Right-side">
                    <CPUModule wasm_name="PC" />
                    <div className="row">
                        <Register wasm_name="ra" />
                        <Register wasm_name="rb" />
                    </div>
                    <CPUModule wasm_name="ALU" />
                    <div className="row">
                        <Register wasm_name="rc" />
                        <Register wasm_name="rd" />
                    </div>
                    {/*  wasm_name="FLAGS"*/}
                    <CPUModule />
                    {/*  wasm_name="SP"*/}
                    <CPUModule />
                    {/*  wasm_name="CW"*/}
                    <CPUModule />
                </div>
            </div>
            <div className="footer"></div>
        </div>
    );
}


export default App;
