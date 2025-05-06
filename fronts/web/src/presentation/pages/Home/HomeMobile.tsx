import React, {FC, ReactElement} from 'react';
import { TextField } from "@mui/material";
import { Button } from '../../components/input/Button/Button';
import { Ramate } from '../../components/icons/ramate/Ramate/Ramate';
import { TaggedTopic, TaggedTopicProps } from '../../assemblies/tagged/TaggedTopic/TaggedTopic';
import { ExpandableSectionDisplay, TitleSubtitleDisplay } from '../../../util';
import useMediaQuery from '@mui/material/useMediaQuery';
import GitHubIcon from '@mui/icons-material/GitHub';
import ArticleIcon from '@mui/icons-material/Article';

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
                    <div style={{
                        display: 'flex',
                        gap: '20px',
                        marginTop: '20px',
                        justifyContent: 'center'
                    }}>
                        <a 
                            href="https://github.com/ramate-io" 
                            target="_blank" 
                            rel="noopener noreferrer"
                            style={{ color: 'inherit' }}
                        >
                            <GitHubIcon sx={{ 
                                fontSize: 32,
                                transition: 'all 0.2s ease-in-out',
                                '&:hover': {
                                    filter: 'drop-shadow(0 0 0.5em rgba(0, 0, 0, 0.3))'
                                }
                            }} />
                        </a>
                        <a 
                            href="https://medium.com/ramate" 
                            target="_blank" 
                            rel="noopener noreferrer"
                            style={{ color: 'inherit' }}
                        >
                            <ArticleIcon sx={{ 
                                fontSize: 32,
                                transition: 'all 0.2s ease-in-out',
                                '&:hover': {
                                    filter: 'drop-shadow(0 0 0.5em rgba(0, 0, 0, 0.3))'
                                }
                            }} />
                        </a>
                    </div>
                </div>
                <div style={{
                    padding : 50,
                    justifySelf : "left",
                    display : "grid",
                    gap : 30
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
        </div>
    )
};