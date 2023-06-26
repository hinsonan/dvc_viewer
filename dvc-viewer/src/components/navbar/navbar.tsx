import Link from 'next/link';

const Navbar: React.FC = () => {
  return (
    <nav className="bg-gray-800 py-4">
      <div className="container mx-auto flex justify-between items-center">
        <Link href="/" className="text-white text-2xl font-bold" passHref>
          DVC Viewer
        </Link>
        <ul className="flex space-x-4">
          <li>
            <Link href="/dataset_viewer" className="text-white" passHref>
              View Datasets
            </Link>
          </li>
        </ul>
      </div>
    </nav>
  );
};

export default Navbar;
