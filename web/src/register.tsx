import React, { useState } from 'react';
import { cpu } from './cpu';
import { CPUModule } from './Modules';

interface regProps {
    wasm_name: string
}

export class Register extends CPUModule<regProps, {}, number> {
    name: string = this.props.wasm_name

    constructor(props: regProps) {
        super(props);
    }
    render() {
        return (
            <div><h3>{this.name.toUpperCase()}</h3>
                <div className="LED" id={this.name}>-</div>
            </div>
        )
    }
}
