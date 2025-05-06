import React, {FC, ReactElement} from 'react';
import { TextField } from "@mui/material";
import { Button } from '../../components/input/Button/Button';
import { Ramate } from '../../components/icons/ramate/Ramate/Ramate';
import { TaggedTopic, TaggedTopicProps } from '../../assemblies/tagged/TaggedTopic/TaggedTopic';
import { ExpandableSectionDisplay, TitleSubtitleDisplay } from '../../../util';
import useMediaQuery from '@mui/material/useMediaQuery';
import GitHubIcon from '@mui/icons-material/GitHub';

export const HOME_DESKTOP_CLASSNAMES : string[] = [ ];
export const HOME_DESKTOP_STYLE : React.CSSProperties = {
    height : "100vh",
    width : "100vw",
    display : "grid",
    alignItems : "center",
    alignContent : "center",
    justifyContent : "center",
    justifyItems : "center",
};

export type HomeDesktopProps = {
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

export const HomeDesktop : FC<HomeDesktopProps>  = (props) =>{

    return (
        <div
        className={[...!props.overrideClasses ? HOME_DESKTOP_CLASSNAMES : [], ...props.classNames||[]].join(" ")}
        style={{...!props.overrideStyle ? HOME_DESKTOP_STYLE : {}, ...props.style}}>
            <div style={{
                display : "grid",
                gridTemplateColumns : "1fr 2fr",
                justifyContent : "center",
                justifyItems : "center",
                alignContent : "center",
                alignItems : "center"
            }}> 
                <div style={{
                    borderRight : "1px solid",
                    justifySelf : "right",
                    padding : 20,
                    textAlign : "center",
                    lineHeight : ".5em"
                }}>
                    <Ramate size={"100%"}/>
                    <h2>
                        ramate
                    </h2>
                    <Button 
                        aonClick={async ()=>{
                            const win : Window = window;
                            win.location = "mailto:liam@ramate.io"
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
                    // lineHeight : "1em",
                    justifySelf : "left",
                    display : "grid",
                    gap : 15
                }}>
                    <h2 style={{
                        margin : 4
                    }}>About</h2>
                    <p style={{
                        fontSize : "1em",
                        margin : 4
                    }}><i>Ramate began as Liam Monninger's single-member LLC for software engineering contracts. It is currently evolving into a collective for esoteric computing projects. We're working on standing up a new flagship project. Check back soon!</i></p>
                </div>
            </div>
            <div style={{
                position: 'absolute',
                bottom: '20px',
                left: '50%',
                transform: 'translateX(-50%)',
                cursor: 'pointer'
            }}>
                <a 
                    href="https://github.com/ramate-io" 
                    target="_blank" 
                    rel="noopener noreferrer"
                    style={{ color: 'inherit' }}
                >
                    <GitHubIcon sx={{ fontSize: 32 }} />
                </a>
            </div>
        </div>
    )
};