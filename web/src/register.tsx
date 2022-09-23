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

        // (REACT)
        // in those constructors add cpu.hash_map entry with wasm_name as key
        // and useState setter as value
        //
        // (RUST)
        // each module's function which changes its internal state
        // should get proper setter
        //
        // each struct will have a pub field wasm_name
        // so it should be easy to check which setter is for it
    }
    render() {
        return (
            <div>register: {this.name}</div>
        )
    }
}
