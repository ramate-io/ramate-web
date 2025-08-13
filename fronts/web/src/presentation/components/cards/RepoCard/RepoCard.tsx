import React, { FC } from 'react';
import { Card, CardActionArea, Typography } from '@mui/material';

export const REPO_CARD_CLASSNAMES: string[] = [];
export const REPO_CARD_STYLE: React.CSSProperties = {
    height: '100%',
    transition: 'all 0.3s ease-in-out',
    cursor: 'pointer',
    borderRadius: '12px',
    boxShadow: '0 2px 8px rgba(0, 0, 0, 0.1)',
};

export type RepoCardProps = {
    children?: React.ReactNode;
    style?: React.CSSProperties;
    overrideStyle?: boolean;
    classNames?: string[];
    overrideClasses?: boolean;
    title: string;
    description: string;
    repoUrl: string;
    iconSrc: string;
    iconAlt?: string;
};

export const RepoCard: FC<RepoCardProps> = (props) => {
    const handleClick = () => {
        window.open(props.repoUrl, '_blank', 'noopener,noreferrer');
    };

    return (
        <Card
            className={[
                ...!props.overrideClasses ? REPO_CARD_CLASSNAMES : [],
                ...props.classNames || []
            ].join(' ')}
            style={{
                ...!props.overrideStyle ? REPO_CARD_STYLE : {},
                ...props.style
            }}
            sx={{
                '&:hover': {
                    transform: 'translateY(-4px)',
                    boxShadow: '0 8px 24px rgba(0, 0, 0, 0.15)',
                    '& .repo-icon': { transform: 'scale(1.1)' },
                    '& .repo-title': { color: '#1976d2' }
                },
                '&:active': { transform: 'translateY(-2px)' }
            }}
        >
            <CardActionArea onClick={handleClick} sx={{ height: '100%' }}>
                <div
                    style={{
                        height: '100%',
                        display: 'grid',
                        gridTemplateRows: '1fr 2fr',
                        gap: 10,
                        padding: 10
                    }}
                >
                    <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                        <div className="repo-icon" style={{ width: 32, height: 32, display: 'flex' }}>
                            <img
                                src={props.iconSrc}
                                alt={props.iconAlt || props.title}
                                style={{ width: '100%', height: '100%', objectFit: 'contain' }}
                            />
                        </div>
                        <Typography
                            variant="h6"
                            component="h3"
                            className="repo-title"
                            sx={{ fontWeight: 600, fontSize: '1.1rem', margin: 0 }}
                        >
                            {props.title}
                        </Typography>
                    </div>

                    <div style={{ height: '100%' }}>
                        <div
                            style={{
                                height: '100%',
                                overflowY: 'auto',
                                paddingTop: 6,
                                paddingRight: 6,
                                paddingBottom: 16,
                                WebkitMaskImage: 'linear-gradient(180deg, black calc(100% - 14px), transparent 100%)',
                                maskImage: 'linear-gradient(180deg, black calc(100% - 14px), transparent 100%)'
                            }}
                        >
                            <Typography variant="body2" color="text.secondary" sx={{ fontSize: '0.9rem', lineHeight: 1.5 }}>
                                {props.description}
                            </Typography>
                        </div>
                    </div>
                </div>
            </CardActionArea>
        </Card>
    );
};
