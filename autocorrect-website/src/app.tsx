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
import { Welcome } from './welcome';

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
          <div className="ml-5 space-x-4">
            <NavbarItem href="/autocorrect/">Intro</NavbarItem>
            <NavbarItem href="/autocorrect/editor">Playground</NavbarItem>
            <NavbarItem
              href="https://github.com/huacnlee/autocorrect"
              target="_blank"
            >
              GitHub
            </NavbarItem>
          </div>
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
