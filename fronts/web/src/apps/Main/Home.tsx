import React, {FC, ReactElement, useEffect, useState} from 'react';
import { Home as HomePage, HomeProps as HomePageProps } from '../../presentation/pages';
import { useNavigate } from 'react-router-dom';
import { EchoerClient, } from "rpc/EchoServiceClientPb";
import * as Echo from 'rpc/Echo_pb';
import { ENGAGEMENTS } from '../../fixtures/engagements';

// const {EchoCom} = Echo;
// const client = new EchoerClient("https://echo-j65oda2z7a-tl.a.run.app:443")

export const HOME_CLASSNAMES : string[] = [ ];
export const HOME_STYLE : React.CSSProperties = {
};

export type HomeProps = {
     children ? : React.ReactNode;
    style ? : React.CSSProperties;
    overrideStyle ? : boolean;
    classNames ? : string[];
    overrideClasses ? : boolean;
    responsive ? : boolean;
};

export const Home : FC<HomeProps>  = (props) =>{


    // tags state
    const [tagsExpanded, setTagsExpanded] = useState<{
        engineering : boolean,
        consulting : boolean,
        research : boolean
    }>({
        engineering : false,
        consulting : false,
        research : false
    });

    const handleEllipses = async (section : string)=>{
        setTagsExpanded({
            ...tagsExpanded,
            [section] : true
        })
    };

    const handleTrim = async (section : string)=>{
        setTagsExpanded({
            ...tagsExpanded,
            [section] : false
        })
    }

    return (
        <HomePage 
        onTrim={handleTrim}
        onEllipses={handleEllipses}
        sectionExpanded={tagsExpanded}
        sections={ENGAGEMENTS}/>
    )
};