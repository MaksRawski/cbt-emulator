import { Tooltip } from "@mui/material";
import React, { ReactNode } from "react"

/**
   * This class isn't meant to be used directly but to be used as a base for other modules.
   * By default it will create a module which holds just one value with `this.name` as ID.
   *
   * With `module()` you can override the "insides" of a module that is, what will be displayed below the module name.
 */
export class CPUModule<P, S> extends React.Component<P, S> {
    name: string = "CPUModule"
    id?: string
    description?: string
    /**
     * Returns a handle for the backend to put binary data in this place.
     */
    led(id: string): ReactNode {
        return <div className="LED" id={id}>-</div>
    }

    module(): ReactNode {
        return this.led(this.id || this.name);
    }

    render(): ReactNode {
        return (
            <div className="module">
                <Tooltip followCursor title={this.description || ""}>
                    <h2 className="module-header">{this.name}</h2>
                </Tooltip>
                {this.module()}
            </div>
        )
    }
}



/**
 * Template for creating basic modules
 */
export class ModuleTemplate extends CPUModule<{ name: string, children?: ReactNode, id?: string, description?: string }, {}>{
    name = this.props.name
    id = this.props.id || this.props.name
    children = this.props.children
    description = this.props.description

    module() {
        return (<div>{this.children} {super.module()}</div>);
    }
}
