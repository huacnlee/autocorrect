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

              <div className="text-lg font-semibold">AutoCorrect</div>
            </Link>
          </div>
          <nav className="ml-5 space-x-4">
            <NavbarItem href="/autocorrect/">Intro</NavbarItem>
            <NavbarItem href="/autocorrect/usage">Usage</NavbarItem>
            <NavbarItem href="/autocorrect/editor">Playground</NavbarItem>
            <NavbarItem
              href="https://github.com/huacnlee/autocorrect"
              target="_blank"
            >
              <span>GitHub</span>
              <ExternalIcon />
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
