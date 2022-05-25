import { GeistProvider, CssBaseline } from "@geist-ui/react";
import type { AppProps } from "next/app";
import "inter-ui/inter.css";

function MyApp({ Component, pageProps }: AppProps) {
    return (
        <GeistProvider themeType="light">
            <CssBaseline />
            <Component {...pageProps} />
        </GeistProvider>
    );
}

export default MyApp;
