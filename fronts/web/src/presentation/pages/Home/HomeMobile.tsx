import React, {FC, ReactElement} from 'react';
import { TextField } from "@mui/material";
import { Button } from '../../components/input/Button/Button';
import { Ramate } from '../../components/icons/ramate/Ramate/Ramate';
import { TaggedTopic, TaggedTopicProps } from '../../assemblies/tagged/TaggedTopic/TaggedTopic';
import { ExpandableSectionDisplay, TitleSubtitleDisplay } from '../../../util';
import useMediaQuery from '@mui/material/useMediaQuery';

export const HOME_MOBILE_CLASSNAMES : string[] = [ ];
export const HOME_MOBILE_STYLE : React.CSSProperties = {
    display : "grid",
    alignItems : "center",
    alignContent : "center",
    justifyContent : "center",
    justifyItems : "center",
};

export type HomeMobileProps = {
     children ? : React.ReactNode;
    style ? : React.CSSProperties;
    overrideStyle ? : boolean;
    classNames ? : string[];
    overrideClasses ? : boolean;
    responsive ? : boolean;
    onPromptChange ? : (prompt : string)=>Promise<void>;
    onLaunch ? : ()=>Promise<void>;
    disabled ? : boolean;
    sections ? : {
        [key : string] : TaggedTopicProps;
    };
    sectionExpanded ? : {
        [key : string] : boolean
    };
    onEllipses ? : (section : string)=>Promise<void>;
    onTrim ? : (section : string)=>Promise<void>;
};

export const HomeMobile : FC<HomeMobileProps>  = (props) =>{

    const _sections = props.sections??{};

    const Sections = Object.entries(_sections).map(([key, value])=>{

        const handleEllipses = async ()=>{
            if(props.onEllipses) await props.onEllipses(key)
        }

        const handleTrim = async ()=>{
            if(props.onTrim) await props.onTrim(key);
        }

        return <TaggedTopic expanded={props.sectionExpanded?.[key]} 
            onTrim={handleTrim}
            onEllipses={handleEllipses} key={key} {...value}/>

    })

    const _handlePromptChange = (event : React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>)=>{
        if(props.onPromptChange)
            props.onPromptChange(event.target.value)
    }

    const matches = useMediaQuery('(min-width:600px)');

    return (
        <div
        className={[...!props.overrideClasses ? HOME_MOBILE_CLASSNAMES : [], ...props.classNames||[]].join(" ")}
        style={{...!props.overrideStyle ? HOME_MOBILE_STYLE : {}, ...props.style}}>
            <div style={{
                display : "grid",
                gridTemplateColumns : "1fr",
                justifyContent : "center",
                justifyItems : "center",
                padding : 20,
            }}> 
                <div style={{
                    padding : 20,
                    textAlign : "center",
                    lineHeight : ".5em",
                    borderBottom : "1px solid",
                    width : "80%"
                }}>
                    <Ramate size={200}/>
                    <h2>
                        ramate
                    </h2>
                    <p>
                        <span style={{
                            opacity : .7,
                        }}>Liam Monninger</span>
                    </p>
                    <Button 
                        aonClick={async ()=>{
                            const win : Window = window;
                            win.location = "mailto:l.mak.monninger@gmail.com"
                        }}
                        variant='contained'
                        sx={{
                            backgroundColor: '#213547',
                            color: 'white',
                            '&:hover': {
                            backgroundColor: '#1a2937',
                            },
                        }}>Inquire</Button>
                </div>
                <div style={{
                    padding : 50,
                    justifySelf : "left",
                    display : "grid",
                    gap : 30
                }}>
                   {Sections}
                </div>
            </div>
        </div>
    )
};