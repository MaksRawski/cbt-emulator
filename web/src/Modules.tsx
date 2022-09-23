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
 * Generics: Props, State, Value
 * - Props and State are passed directly to react
 * - Value is type of data used internally by a given module.
 */
export class CPUModule<P, S, V> extends React.Component<P, S> {
    setter: reactDummySetter<V> = () => { };

    /* checkSetter(module_name: string) {
*     let modSetter = global.setters.get(module_name);
*     if (modSetter !== undefined) {
*         this.setter = modSetter;
*     } else {
*         console.error("Missing setter for %s!", module_name);
*     }
* } */
    render() {
        return <div>CPUModule</div>
    }
}
