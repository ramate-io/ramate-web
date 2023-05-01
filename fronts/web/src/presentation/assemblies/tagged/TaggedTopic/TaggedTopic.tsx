import React, {FC, ReactElement, createRef, useRef, useMemo, useReducer, useEffect, useState} from 'react';
import { Chip } from "@mui/material";
import { InputChip } from '../../../components/input/InputChip/InputChip';
import { ShortDisplayCharacter } from '../../../../util';

export const TAGGED_TOPIC_CLASSNAMES : string[] = [ ];
export const TAGGED_TOPIC_STYLE : React.CSSProperties = {
};

export type TaggedTopicProps = {
     children ? : React.ReactNode;
    style ? : React.CSSProperties;
    overrideStyle ? : boolean;
    classNames ? : string[];
    overrideClasses ? : boolean;
    responsive ? : boolean;
    tags ? : {
        [key : string] : ShortDisplayCharacter
    };
    title ? : string;
    description ? : string;
    maxRows ? : number;
    expanded ? : boolean;
    onEllipses ? : ()=>Promise<void>;
    onTrim ? : ()=>Promise<void>;
    onTagClick ? : (state : {
        key : string,
        character : ShortDisplayCharacter;
    })=>Promise<void>;
};

export const TaggedTopic : FC<TaggedTopicProps>  = (props) =>{

    // defaults
    const _maxRows = props.maxRows??2;
    const _tags = props.tags??{};
    const _expanded = props.expanded??false;

    // tag refs
    const tagRefs = useRef<React.RefObject<HTMLDivElement>[]>(
        Array(Object.keys(_tags).length)
        .fill(undefined).map(()=>createRef<HTMLDivElement>())
    );

    // manual render management
    const [tick, increment] = useReducer(x=>x+1, 0);

    // tag limit
    const [limit, setLimit] = useState(tagRefs.current.length);

    const Tags = useMemo(()=>{

        const chips = Object.keys(_tags).map((key, i)=>{

            const display = _tags[key].displayText;
            const selected = _tags[key].selected;
            const ref = tagRefs.current[i]; 

            return <div key={key} ref={ref}>
                <Chip clickable key={key} label={display} variant={selected ? "filled" : "outlined"}/>
            </div>

        })

        increment();
        return chips;

    }, []);

    const evaluateSize = ()=>{

        const rows : {[key : number]  : number[]}= {};
        let last = 0;
        for(const [i, el] of tagRefs.current.entries()){

            if(el.current && !rows[el.current.offsetTop]){
                rows[el.current.offsetTop] = []
                last = el.current.offsetTop;
            }

            if(el.current?.offsetTop)
                rows[el.current.offsetTop].push(i);
            else rows[last].push(i); // put all of the unrendered content at the end

        }

        const size = Object.keys(rows).length;
        const endKey : number = Object.keys(rows)[_maxRows] !== undefined ? 
        Object.keys(rows)[_maxRows] as unknown as number
        : tagRefs.current.length;
        //  const endRow = Object.keys()
        console.log(rows, endKey, size <= _maxRows, limit, endKey, limit < (endKey - 1))
        if(endKey && size > _maxRows){
            setLimit(rows[endKey][0]);
        }

        if (
            (size <= _maxRows) 
            && (limit < (endKey - 1))
        ){
            increment();
            setLimit(limit + 1);
        }
            

    }

    useEffect(()=>{

        evaluateSize();

        const listener = ()=>increment();

        window.addEventListener("resize", listener);

        return ()=>{
            window.removeEventListener("resize", listener);
        }

    }, [tick])

    const handleEllipses = ()=>{
        if(props.onEllipses) props.onEllipses()
    }

    const handleTrim = ()=>{
        if(props.onTrim) props.onTrim();
    }

    return (
        <div
        className={[...!props.overrideClasses ? TAGGED_TOPIC_CLASSNAMES : [], ...props.classNames||[]].join(" ")}
        style={{...!props.overrideStyle ? TAGGED_TOPIC_STYLE : {}, ...props.style}}>
            <h2>{props.title}</h2>
            <p style={{
                fontSize : "1em"
            }}><i>{props.description}</i></p>
            <div style={{
                display : "flex",
                gap : 2,
                flexWrap : "wrap",
            }}>
                {_expanded ? Tags : Tags.slice(0, limit)}
                {(
                    !_expanded
                    && (limit !== tagRefs.current.length)
                ) && <Chip 
                    onClick={handleEllipses} 
                    clickable key={"elip"} label={"..."} variant={"outlined"}
                />}
                {_expanded && <InputChip 
                    clickable key={"suggest"} label={"Your need"} variant={"outlined"}/>}
                {(
                    _expanded
                    && (limit !== tagRefs.current.length)
                ) && <Chip 
                    onClick={handleTrim} 
                    clickable key={"trim"} label={"-/---"} variant={"outlined"}
                />}
            </div>
        </div>
    )
};