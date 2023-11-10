/// <reference lib="dom" />
/// <reference lib="dom.iterable" />

import React from 'react';
import { createRoot } from 'react-dom/client';
import {
  Link,
  NavLink,
  Outlet,
  RouterProvider,
  createBrowserRouter,
} from 'react-router-dom';
import { AppEditor } from './AppEditor';
import { UsagePage } from './usage';
import { Welcome } from './welcome';

const ExternalIcon = () => {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      width="12"
      height="12"
      className="fill-gray-400"
    >
      <path d="M10 6V8H5V19H16V14H18V20C18 20.5523 17.5523 21 17 21H4C3.44772 21 3 20.5523 3 20V7C3 6.44772 3.44772 6 4 6H10ZM21 3V11H19L18.9999 6.413L11.2071 14.2071L9.79289 12.7929L17.5849 5H13V3H21Z"></path>
    </svg>
  );
};

const Layout = () => {
  const NavbarItem = (props: any) => {
    const { href, children } = props;

    if (href.startsWith('/autocorrect/')) {
      return (
        <NavLink
          className={({ isActive, isPending }) => {
            return isActive ? 'navbar-item navbar-item-active' : 'navbar-item';
          }}
          to={href}
        >
          {children}
        </NavLink>
      );
    }

    return (
      <a className="navbar-item" {...props}>
        {children}
      </a>
    );
  };

  return (
    <>
      <div className="navbar">
        <div className="flex items-center justify-between px-4">
          <div className="navbar-brand">
            <Link to="/autocorrect/" className="flex items-center space-x-2">
              <img
                src="https://user-images.githubusercontent.com/5518/194691346-13856309-266b-4bf6-b505-5a8b15d0c02e.png"
                className="h-8"
              />

              <div className="hidden text-lg font-semibold md:block">
                AutoCorrect
              </div>
            </Link>
          </div>
          <nav className="flex items-center ml-5 space-x-4">
            <NavbarItem href="/autocorrect/">Intro</NavbarItem>
            <NavbarItem href="/autocorrect/usage">Usage</NavbarItem>
            <NavbarItem href="/autocorrect/editor">Playground</NavbarItem>
            <NavbarItem
              href="https://github.com/huacnlee/autocorrect"
              target="_blank"
              className="p-0"
            >
              <svg
                className="w-5 h-5 fill-gray-600 dark:fill-gray-400 hover:opacity-60"
                role="img"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"></path>
              </svg>
            </NavbarItem>
          </nav>
        </div>
      </div>
      <Outlet />
    </>
  );
};

const router = createBrowserRouter([
  {
    path: '/autocorrect',

    element: <Layout />,
    children: [
      {
        path: '',
        index: true,
        element: <Welcome />,
      },
      {
        path: '/autocorrect/usage',
        element: <UsagePage />,
      },
      {
        path: '/autocorrect/editor',
        element: <AppEditor />,
      },
    ],
  },
]);

document.addEventListener('DOMContentLoaded', () => {
  const appEditor = createRoot(document.getElementById('app') as any);
  appEditor.render(
    <React.StrictMode>
      <RouterProvider router={router} />
    </React.StrictMode>
  );
});
