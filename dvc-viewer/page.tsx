'use client'
import { useEffect } from 'react';
import { useRouter } from 'next/router';

const SplashPage: React.FC = () => {
  const router = useRouter();

  useEffect(() => {
    // Simulating a loading delay
    const timer = setTimeout(() => {
      router.push('/mainpage'); // Replace '/home' with the path to your main page
    }, 2000);

    return () => clearTimeout(timer);
  }, [router]);

  return (
    <div className="flex justify-center items-center h-screen">
      <h1 className="text-4xl font-bold">Splash Page</h1>
    </div>
  );
};

export default SplashPage;
