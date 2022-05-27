import Link from "next/link";
import { EuiHeader, EuiTitle, useEuiTheme, EuiHeaderLinks, EuiHeaderLink } from "@elastic/eui";
import ThemeSwitcher from "./theme_switcher";
import { headerStyles } from "./header.styles";

const Header = () => {
    const { euiTheme } = useEuiTheme();
    const styles = headerStyles(euiTheme);

    return (
        <EuiHeader
            sections={[
                {
                    items: [
                        <Link key="logo-eui" href="/" passHref>
                            <a css={styles.logo}>
                                <EuiTitle size="xxs" css={{ fontWeight: "500 !important" }}>
                                    <span>UNSW Security Challenges Platform</span>
                                </EuiTitle>
                            </a>
                        </Link>,
                        <EuiHeaderLinks key="links">
                            <EuiHeaderLink isActive>Challenges</EuiHeaderLink>
                        </EuiHeaderLinks>,
                    ],
                    borders: "none",
                },
                {
                    items: [<ThemeSwitcher key="theme-switcher" />],
                    borders: "none",
                },
            ]}
        />
    );
};

export default Header;
