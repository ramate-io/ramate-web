import React, {FC, ReactElement, useState} from 'react';
import { Chip, ChipProps, TextField } from "@mui/material";

export const INPUT_CHIP_CLASSNAMES : string[] = [ ];
export const INPUT_CHIP_STYLE : React.CSSProperties = {
};

export type InputChipProps = ChipProps & {
     children ? : React.ReactNode;
    style ? : React.CSSProperties;
    overrideStyle ? : boolean;
    classNames ? : string[];
    overrideClasses ? : boolean;
    responsive ? : boolean;
};

export const InputChip : FC<InputChipProps>  = (props) =>{

    const [focused, setFocused] = useState(false);

    return (
       <Chip {...props} sx={{
        overflow : "hidden"
       }} label={<TextField
            onBlur={(e)=>setFocused(false)}
            onFocus={(e)=>setFocused(true)}
            InputLabelProps={{
                sx : {
                    // display : "none",
                    fontSize : 14
                },
                className : focused ? "fade-out" : undefined
            }}
            sx={{ 
                '& legend': { display: 'none' }, '& fieldset': { bottom: 0, border : "none" },
                border: 'none',
                '.MuiInputBase-input': { fontSize: 14 },
            }} 
            // variant='standard'
            size='small' label={props.label}/>}/>
    )
};