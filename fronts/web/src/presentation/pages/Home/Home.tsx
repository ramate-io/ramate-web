import React, {FC, ReactElement} from 'react';
import { TextField } from "@mui/material";
import { Button } from '../../components/input/Button/Button';
import { Ramate } from '../../components/icons/ramate/Ramate/Ramate';
import { TaggedTopic, TaggedTopicProps } from '../../assemblies/tagged/TaggedTopic/TaggedTopic';

export const HOME_CLASSNAMES : string[] = [ ];
export const HOME_STYLE : React.CSSProperties = {
    height : "100vh",
    width : "100vw",
    display : "grid",
    alignItems : "center",
    alignContent : "center",
    justifyContent : "center",
    justifyItems : "center",
};

export type HomeProps = {
     children ? : React.ReactNode;
    style ? : React.CSSProperties;
    overrideStyle ? : boolean;
    classNames ? : string[];
    overrideClasses ? : boolean;
    responsive ? : boolean;
    onPromptChange ? : (prompt : string)=>Promise<void>;
    onLaunch ? : ()=>Promise<void>;
    disabled ? : boolean;
    engineeringTags ? : TaggedTopicProps["tags"];
    consultingTags ? : TaggedTopicProps["tags"];
    researchTags ? : TaggedTopicProps["tags"];
};

export const Home : FC<HomeProps>  = (props) =>{

    const _handlePromptChange = (event : React.ChangeEvent<HTMLTextAreaElement | HTMLInputElement>)=>{
        if(props.onPromptChange)
            props.onPromptChange(event.target.value)
    }

    return (
        <div
        className={[...!props.overrideClasses ? HOME_CLASSNAMES : [], ...props.classNames||[]].join(" ")}
        style={{...!props.overrideStyle ? HOME_STYLE : {}, ...props.style}}>
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
                    <Ramate/>
                    <h2>
                        ramate
                    </h2>
                    <p>
                        <span style={{
                            opacity : .7,
                        }}>Liam Monninger</span>
                    </p>
                </div>
                <div style={{
                    padding : 50,
                    lineHeight : "1em",
                    justifySelf : "left"
                }}>
                    <TaggedTopic
                        maxRows={1}
                        title='Software engineering'
                        description='Starting from $60/hr'
                        tags={{}}
                        />
                    <br/>
                    <TaggedTopic
                        expanded
                        maxRows={1}
                        title='Software consulting and system design'
                        description='Starting from $45/hr'
                        tags={{}}
                    />
                    <br/>
                    <TaggedTopic
                        maxRows={1}
                        title='Research, data science, GIS, and content'
                        description='Starting from $35/hr'
                        tags={{}}
                    />
                </div>
            </div>
        </div>
    )
};