import * as React from "react";
import { NavLink, useLocation, useHistory } from "react-router-dom";
import {
  Nav,
  NavList,
  NavItem,
  NavExpandable,
  Page,
  PageHeader,
  PageSidebar,
  SkipToContent,
  Dropdown, DropdownToggle, Avatar, DropdownGroup, DropdownItem, PageHeaderTools
} from "@patternfly/react-core";
import { routes, IAppRoute, IAppRouteGroup } from "@app/routes";
import { useAuth } from "oidc-react";
import logo from "@app/bgimages/Patternfly-Logo.svg";
import MD5 from 'crypto-js/md5';

interface IAppLayout {
  children: React.ReactNode;
}

const AppLayout: React.FunctionComponent<IAppLayout> = ({ children }) => {
  const [isNavOpen, setIsNavOpen] = React.useState(true);
  const [isMobileView, setIsMobileView] = React.useState(true);
  const [isNavOpenMobile, setIsNavOpenMobile] = React.useState(false);
  const onNavToggleMobile = () => {
    setIsNavOpenMobile(!isNavOpenMobile);
  };
  const onNavToggle = () => {
    setIsNavOpen(!isNavOpen);
  };
  const onPageResize = (props: { mobileView: boolean; windowSize: number }) => {
    setIsMobileView(props.mobileView);
  };
  const [isDropdownOpen, setIsDropdownOpen] = React.useState(false);
  const onDropdownToggle = () => {
    setIsDropdownOpen(!isDropdownOpen);
  };
  const onSelectDropdown = () => {
    setIsDropdownOpen(true);
  };


  function LogoImg() {
    const history = useHistory();

    function handleClick() {
      history.push("/");
    }

    return (
      <img src={logo} onClick={handleClick} alt="PatternFly Logo" />
    );
  }

  const auth = useAuth();

  const userDropdownItems = [
    <DropdownGroup key="userGroup1">
      {
        (auth && auth.userData) && (<DropdownItem key="userGroup1-logout" onClick={() => {
          console.log("Log out");
          return auth.signOut();
        }}>Logout</DropdownItem>)
      }
    </DropdownGroup>
  ];

  const headerTools = () => {

    const h = MD5(auth.userData?.profile.email).toString();
    const avatarUrl = `https://www.gravatar.com/avatar/${h}?D=mp`;

    return (
      <Dropdown
        position="right"
        onSelect={onSelectDropdown}
        isOpen={isDropdownOpen}
        isPlain
        toggle={
          <DropdownToggle className="user-toggle" icon={<Avatar src={avatarUrl} alt="Avatar" />}
                          onToggle={onDropdownToggle}>
            {auth.userData?.profile.preferred_username}
          </DropdownToggle>
        }
        dropdownItems={userDropdownItems}
      />
    );
  };

  const Header = (
    <PageHeader
      logo={<LogoImg />}
      showNavToggle
      isNavOpen={isNavOpen}
      onNavToggle={isMobileView ? onNavToggleMobile : onNavToggle}
      headerTools={<PageHeaderTools>{headerTools()}</PageHeaderTools>}
    />
  );

  const location = useLocation();

  const renderNavItem = (route: IAppRoute, index: number) => (
    <NavItem key={`${route.label}-${index}`} id={`${route.label}-${index}`} isActive={route.path === location.pathname}>
      <NavLink exact={route.exact} to={route.path}>
        {route.label}
      </NavLink>
    </NavItem>
  );

  const renderNavGroup = (group: IAppRouteGroup, groupIndex: number) => (
    <NavExpandable
      key={`${group.label}-${groupIndex}`}
      id={`${group.label}-${groupIndex}`}
      title={group.label}
      isActive={group.routes.some((route) => route.path === location.pathname)}
    >
      {group.routes.map((route, idx) => route.label && renderNavItem(route, idx))}
    </NavExpandable>
  );

  const Navigation = (
    <Nav id="nav-primary-simple" theme="dark">
      <NavList id="nav-list-simple">
        {routes.map(
          (route, idx) => route.label && (!route.routes ? renderNavItem(route, idx) : renderNavGroup(route, idx))
        )}
      </NavList>
    </Nav>
  );

  const Sidebar = (
    <PageSidebar
      theme="dark"
      nav={Navigation}
      isNavOpen={isMobileView ? isNavOpenMobile : isNavOpen} />
  );

  const pageId = "primary-app-container";

  const PageSkipToContent = (
    <SkipToContent onClick={(event) => {
      event.preventDefault();
      const primaryContentContainer = document.getElementById(pageId);
      primaryContentContainer && primaryContentContainer.focus();
    }} href={`#${pageId}`}>
      Skip to Content
    </SkipToContent>
  );
  return (
    <Page
      mainContainerId={pageId}
      header={Header}
      sidebar={Sidebar}
      onPageResize={onPageResize}
      skipToContent={PageSkipToContent}>
      {children}
    </Page>
  );
};

export { AppLayout };
