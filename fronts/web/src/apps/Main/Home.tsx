import React, {FC, ReactElement, useEffect, useState} from 'react';
import { Home as HomePage, HomeProps as HomePageProps } from '../../presentation/pages';
import { startModel } from '../../data/deployer/startModel';
import { useNavigate } from 'react-router-dom';
import { EchoerClient, } from "rpc/EchoServiceClientPb";
import * as Echo from 'rpc/Echo_pb';

const {EchoCom} = Echo;
const client = new EchoerClient("https://echo-j65oda2z7a-tl.a.run.app:443")

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

    useEffect(()=>{

        const request = new EchoCom();
        request.setText("world");
        client.call(request, {})
        .then((res)=>{
            console.log(res.getText())
        })

    }, [])

    // tags state
    const [tagsExpanded, setTagsExpanded] = useState<{
        engineering : boolean,
        consulting : boolean,
        research : boolean
    }>({
        engineering : false,
        consulting : false,
        research : false
    })

    return (
        <HomePage sections={{
            "engineering" : {
                title : "Software engineering",
                description : "Software engineering starting from $60",
                tags : {
                    "frontend" : {
                        displayText : "frontend",
                        color : "black",
                        selected : false
                    },
                    "backend" : {
                        displayText : "backend",
                        color : "black",
                        selected : false
                    },
                    "redis" : {
                        displayText : "redis",
                        color : "black",
                        selected : false
                    },
                    "rust" : {
                        displayText : "rust",
                        color : "black",
                        selected : false
                    }, 
                    "react" :  {
                        displayText : "React.js",
                        color : "black",
                        selected : false
                    },
                    "python" :  {
                        displayText : "Python",
                        color : "black",
                        selected : false
                    },
                    "torch" :  {
                        displayText : "PyTorch",
                        color : "black",
                        selected : false
                    },
                    "infrastructure" :  {
                        displayText : "infrastructure",
                        color : "black",
                        selected : false
                    },
                    "terraform" :  {
                        displayText : "Terraform",
                        color : "black",
                        selected : false
                    },
                    "spark" :  {
                        displayText : "Spark",
                        color : "black",
                        selected : false
                    },
                    "flink" :  {
                        displayText : "Flink",
                        color : "black",
                        selected : false
                    },
                    "aws" :  {
                        displayText : "AWS",
                        color : "black",
                        selected : false
                    },
                    "gcp" :  {
                        displayText : "GCP",
                        color : "black",
                        selected : false
                    },
                }
            },
            "next" : {
                title : "Software engineering",
                description : "Software engineering starting from $60",
                tags : {
                    "frontend" : {
                        displayText : "frontend",
                        color : "black",
                        selected : false
                    },
                    "backend" : {
                        displayText : "backend",
                        color : "black",
                        selected : false
                    },
                    "redis" : {
                        displayText : "redis",
                        color : "black",
                        selected : false
                    },
                    "rust" : {
                        displayText : "rust",
                        color : "black",
                        selected : false
                    }, 
                    "react" :  {
                        displayText : "React.js",
                        color : "black",
                        selected : false
                    },
                    "python" :  {
                        displayText : "Python",
                        color : "black",
                        selected : false
                    },
                    "torch" :  {
                        displayText : "PyTorch",
                        color : "black",
                        selected : false
                    },
                    "infrastructure" :  {
                        displayText : "infrastructure",
                        color : "black",
                        selected : false
                    },
                    "terraform" :  {
                        displayText : "Terraform",
                        color : "black",
                        selected : false
                    },
                    "spark" :  {
                        displayText : "Spark",
                        color : "black",
                        selected : false
                    },
                    "flink" :  {
                        displayText : "Flink",
                        color : "black",
                        selected : false
                    },
                    "aws" :  {
                        displayText : "AWS",
                        color : "black",
                        selected : false
                    },
                    "gcp" :  {
                        displayText : "GCP",
                        color : "black",
                        selected : false
                    },
                }
            },
            "last" : {
                title : "Software engineering",
                description : "Software engineering starting from $60",
                tags : {
                    "frontend" : {
                        displayText : "frontend",
                        color : "black",
                        selected : false
                    },
                    "backend" : {
                        displayText : "backend",
                        color : "black",
                        selected : false
                    },
                    "redis" : {
                        displayText : "redis",
                        color : "black",
                        selected : false
                    },
                    "rust" : {
                        displayText : "rust",
                        color : "black",
                        selected : false
                    }, 
                    "react" :  {
                        displayText : "React.js",
                        color : "black",
                        selected : false
                    },
                    "python" :  {
                        displayText : "Python",
                        color : "black",
                        selected : false
                    },
                    "torch" :  {
                        displayText : "PyTorch",
                        color : "black",
                        selected : false
                    },
                    "infrastructure" :  {
                        displayText : "infrastructure",
                        color : "black",
                        selected : false
                    },
                    "terraform" :  {
                        displayText : "Terraform",
                        color : "black",
                        selected : false
                    },
                    "spark" :  {
                        displayText : "Spark",
                        color : "black",
                        selected : false
                    },
                    "flink" :  {
                        displayText : "Flink",
                        color : "black",
                        selected : false
                    },
                    "aws" :  {
                        displayText : "AWS",
                        color : "black",
                        selected : false
                    },
                    "gcp" :  {
                        displayText : "GCP",
                        color : "black",
                        selected : false
                    },
                }
            }
        }}/>
    )
};