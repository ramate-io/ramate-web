import React, {FC, ReactElement} from 'react';
import { TextField } from "@mui/material";
import { Button } from '../../components/input/Button/Button';
import { Ramate } from '../../components/icons/ramate/Ramate/Ramate';
import { TaggedTopic, TaggedTopicProps } from '../../assemblies/tagged/TaggedTopic/TaggedTopic';
import { ExpandableSectionDisplay, TitleSubtitleDisplay } from '../../../util';
import useMediaQuery from '@mui/material/useMediaQuery';
import { HomeDesktop } from './HomeDesktop';
import { HomeMobile } from './HomeMobile';

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
    sections ? : {
        [key : string] : TaggedTopicProps;
    };
    sectionExpanded ? : {
        [key : string] : boolean
    };
    onEllipses ? : (section : string)=>Promise<void>;
    onTrim ? : (section : string)=>Promise<void>;
};

export const Home : FC<HomeProps>  = (props) =>{

    const matches = useMediaQuery('(min-width:600px)');

    return (
       matches ? <HomeDesktop {...props}/> : <HomeMobile {...props}/>
    )
};