import React, {FC, ReactElement} from 'react';
import { TextField } from "@mui/material";
import { Button } from '../../components/input/Button/Button';
import { Ramate } from '../../components/icons/ramate/Ramate/Ramate';
import { RepoCard } from '../../components/cards/RepoCard/RepoCard';
import { TaggedTopic, TaggedTopicProps } from '../../assemblies/tagged/TaggedTopic/TaggedTopic';
import { ExpandableSectionDisplay, TitleSubtitleDisplay } from '../../../util';
import useMediaQuery from '@mui/material/useMediaQuery';
import GitHubIcon from '@mui/icons-material/GitHub';
import ArticleIcon from '@mui/icons-material/Article';
import ramateIconLight from '../../../assets/ramate-transparent.png';
import ramateIconDark from '../../../assets/ramate-inverted-transparent.png';
import oacIconLight from '../../../assets/oac-transparent.png';
import oacIconDark from '../../../assets/oac-inverted-transparent.png';
import roblesIconLight from '../../../assets/robles-transparent.png';
import roblesIconDark from '../../../assets/robles-inverted-transparent.png';

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

    const isDark = useMediaQuery('(prefers-color-scheme: dark)');
    const ramateIconSrc = isDark ? ramateIconDark : ramateIconLight;
    const oacIconSrc = isDark ? oacIconDark : oacIconLight;
    const roblesIconSrc = isDark ? roblesIconDark : roblesIconLight;

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
                    gap : 25
                }}>
                    <div>
                        <h2 style={{
                            margin : 4
                        }}>About</h2>
                        <p style={{
                            fontSize : "1em",
                            margin : 4
                        }}><i>Ramate is embarking on a mission to become a computing collective, with a flagship project in distributed and decentralized systems!</i></p>
                    </div>
                    
                    <div>
                        <h3 style={{
                            margin: '4px 4px 16px 4px',
                            fontSize: '1.1rem'
                        }}>Our Projects</h3>
                        <div style={{
                            display: 'grid',
                            gridTemplateColumns: 'repeat(2, 1fr)',
                            gridAutoRows: '1fr',
                            gap: '30px'
                        }}>
                            <RepoCard
                                title="Ramate"
                                description="The organization with many branches. Main company repository containing artifacts and governance."
                                repoUrl="https://github.com/ramate-io/ramate"
                                 iconSrc={ramateIconSrc}
                                iconAlt="Ramate Logo"
                            />
                            <RepoCard
                                title="OAC"
                                description="Ordered Atomic Collaboration - a paradigm for decentralized consequence."
                                repoUrl="https://github.com/ramate-io/oac"
                                 iconSrc={oacIconSrc}
                                iconAlt="OAC Logo"
                            />
                            <RepoCard
                                title="Robles"
                                description="Ramate's stack for programming distributed and decentralized systems with OAC."
                                repoUrl="https://github.com/ramate-io/robles"
                                 iconSrc={roblesIconSrc}
                                iconAlt="Robles Logo"
                            />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
};