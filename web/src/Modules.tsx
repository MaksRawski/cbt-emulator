import React from "react"

export type reactSetter<V> = React.Dispatch<React.SetStateAction<V>>
export type reactDummySetter<V> = (reactSetter<V> | (() => void));

let off = '○';
let on = '●';

export function to_binary(n: number, width: number): string {
    let res = "";
    for (let i = 0; i < width; i++) {
        res += n & (1 << i) ? on : off;
    }
    return res;
}

/**
   * This class isn't meant to be used directly but to be used as a base for other modules.
   * By default it will create a module which holds just one value with `this.name` as ID.
   *
   * With `module()` you can override the "insides" of a module that is, what will be displayed below the module name.
 */
export class CPUModule<P> extends React.Component<P, {}> {
    name: string = "CPUModule"
    id?: string
    /**
     * Returns a handle for the backend to put binary data in this place.
     */
    led_num(id: string) {
        return <div className="LED" id={id}>-</div>
    }

    module() {
        return this.led_num(this.id || this.name);
    }

    render() {
        return (
            <div className="module">
                <h2 className="module-header">{this.name}</h2>
                {this.module()}
            </div>
        )
    }
}



/**
 * Template for creating basic modules
 */
export class ModuleTemplate extends CPUModule<{ name: string, id?: string }>{
    name = this.props.name
    id = this.props.id
}
