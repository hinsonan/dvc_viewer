import * as React from 'react';
import { useEffect, useState } from 'react';
import Accordion from '@mui/material/Accordion';
import AccordionSummary from '@mui/material/AccordionSummary';
import AccordionDetails from '@mui/material/AccordionDetails';
import Typography from '@mui/material/Typography';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import { createTheme, ThemeProvider } from '@mui/material/styles';

// Create a custom dark theme
const darkTheme = createTheme({
    palette: {
        mode: 'dark',
    },
});


export interface DataItem {
    title: string;
    path: string;
    md5: string;
    size: number;
    nfiles: number | null;
    desc: string;
    remote: string;
}

interface DataListProps {
    data: DataItem[];
}

export default function SimpleAccordion({ data }: DataListProps) {
    const [mounted, setMounted] = useState(false);

    useEffect(() => {
        setMounted(true);
    }, []);

    return (
        <div>
            {data.map((item, index) => (
                <div className={`p-1 accordion-container ${mounted ? 'fade-in' : ''} accord`} key={index}>
                    <ThemeProvider theme={darkTheme}>
                        <Accordion>
                            <AccordionSummary
                                expandIcon={<ExpandMoreIcon />}
                                aria-controls="panel1a-content"
                                id="panel1a-header"
                            >
                                <Typography>{item.title.replace("dataset-registry/","")}</Typography>
                            </AccordionSummary>
                            <AccordionDetails>
                                <Typography>
                                    <ul>
                                        <li>Path: {item.path}</li>
                                        <li>MD5: {item.md5}</li>
                                        <li>Size: {item.size ? (item.size / (1024 * 1024 * 1024)).toFixed(2).toString() + " GB" : "N/A"}</li>
                                        <li>Number of Files: {item.nfiles}</li>
                                        <li>Description: {item.desc}</li>
                                        <li>Remote: {item.remote}</li>
                                    </ul>
                                </Typography>
                            </AccordionDetails>
                        </Accordion>
                    </ThemeProvider>
                </div>
            ))}
            <style jsx>{`
        .accordion-container {
            opacity: 0;
          }
        .fade-in {
            animation: fade 1s ease-in-out forwards;
          }
  
          @keyframes fade {
            from {
              opacity: 0;
            }
            to {
              opacity: 1;
            }
          }
      `}</style>
        </div>
    );
}