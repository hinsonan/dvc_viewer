'use client'
import { useState, useEffect } from 'react';
import SimpleAccordion from '@/components/accordian/accordian';
import BasicButton from '@/components/button/button';
import Navbar from '@/components/navbar/navbar';
import BasicTextFields from '@/components/text_input/basic_text_input';

export default function Home() {
  const [toggle, setData] = useState<number>(0);

  const handleClick = () => {
    console.log('Custom button clicked');
    // Perform any other action or logic here
    const eventSource = new EventSource('http://localhost:8000/events');

    eventSource.onmessage = (event) => {
      const data = event.data;
      console.log(data);
      if (data === "ping"){
        eventSource.close();
        toggle > 0 ? setData(0): setData(1);
      }
    };

    eventSource.onerror = (error) => {
      console.error(error);
    };

    // Clean up the EventSource when the component unmounts
    return () => {
      eventSource.close();
    };
  };

  return (
    <div>
      <Navbar />
      <main className="flex flex-col items-center justify-between p-24">
        <div>
          <BasicTextFields />
          <div className="flex flex-col float-right">
            <BasicButton onClick={handleClick}/>
          </div>
        </div>
        {toggle !== 0 && <SimpleAccordion />}
      </main>
    </div>
  )
}
