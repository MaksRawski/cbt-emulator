import React, { useState } from 'react';
import './css/App.scss';
import { CLK } from './clk';
import { Register } from './register';
import { reactSetter, CPUModule } from './Modules';

const App = () => {
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
                    <Register wasm_name="ir" />
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
                    <Register wasm_name="sp" />
                    {/*  wasm_name="CW"*/}
                    <CPUModule />
                </div>
            </div>
            <div className="footer"></div>
        </div>
    );
}


export default App;
