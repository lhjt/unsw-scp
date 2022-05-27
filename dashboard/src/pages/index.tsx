import { FunctionComponent } from "react";
import Head from "next/head";
import { EuiAccordion, EuiCallOut, EuiSpacer, EuiText } from "@elastic/eui";
import Header from "../components/starter/header";
import FlagCard from "../components/challenges/FlagCard";
import { GetServerSideProps } from "next/types";

type NewService = {
    id: BigInt;
    category: string;
    name: string;
    not_before?: Date;
    not_after?: Date;
};

type NewFlag = {
    id: string;
    flag_type: string;
    display_name: string;
    category: string;
    points: number;
};

export const getServerSideProps: GetServerSideProps = async context => {
    const token = context.req.headers["x-scp-auth"] as string;

    const id = await (
        await fetch(`http://${process.env.GAIA_ADDR ?? "gaia-backend:8081"}/api/selfserve/id`, {
            headers: {
                "x-scp-auth": token,
            },
        })
    ).text();

    const roles = (await (
        await fetch(`http://${process.env.GAIA_ADDR ?? "gaia-backend:8081"}/api/selfserve/roles`, {
            headers: {
                "x-scp-auth": token,
            },
        })
    ).json()) as string[];

    const challengeData = (await (
        await fetch(`http://${process.env.ROUTER_URL ?? "router:8082"}/api/challenges`, {
            headers: {
                "x-scp-auth": token,
            },
        })
    ).json()) as {
        id: BigInt;
        services: NewService[];
        flags: NewFlag[];
    }[];

    return {
        props: { id, roles, challengeData },
    };
};

const Index: FunctionComponent<{
    id: string;
    roles: string[];
    challengeData: {
        id: BigInt;
        services: NewService[];
        flags: NewFlag[];
    }[];
}> = ({ id, roles, challengeData }) => {
    const categoryMap: {
        [key: string]: {
            services: {
                id: BigInt;
                category: string;
                name: string;
                not_before?: Date;
                not_after?: Date;
                challenge: BigInt;
            }[];
            flags: {
                id: string;
                flag_type: string;
                display_name: string;
                category: string;
                points: number;
                challenge: BigInt;
            }[];
        };
    } = {};

    for (const chal of challengeData) {
        for (const service of chal.services) {
            if (!categoryMap[service.category]) {
                categoryMap[service.category] = {
                    services: [],
                    flags: [],
                };
            }

            categoryMap[service.category].services.push({ ...service, challenge: chal.id });
        }

        for (const flag of chal.flags) {
            if (!categoryMap[flag.category]) {
                categoryMap[flag.category] = {
                    services: [],
                    flags: [],
                };
            }

            categoryMap[flag.category].flags.push({ ...flag, challenge: chal.id });
        }
    }

    const categories = Object.keys(categoryMap);

    return (
        <>
            <Head>
                <title>UNSW SCP - Challenges</title>
            </Head>

            <Header />
            <div css={{ margin: "1rem" }}>
                <EuiText>
                    <h1>Challenges for {id}</h1>
                </EuiText>
                <EuiSpacer />
                <EuiText>
                    <p>
                        The challenges that are visible are available for you to attempt to
                        complete. Upon successfully completing a challenge, you will be supplied
                        with a flag that you should submit to the relevant form on this page.
                    </p>
                </EuiText>
                <EuiSpacer />
                <EuiCallOut size="m" title="Dynamic Flags" iconType="pin">
                    <p>
                        Some flags are dynamic. This means that they are independently generated for
                        your account, and will not work for other people.
                    </p>
                </EuiCallOut>
                <EuiSpacer />
                {categories.sort().map(category => (
                    <EuiAccordion
                        key={category}
                        id="main-accord"
                        className="euiAccordionForm"
                        element="fieldset"
                        buttonContent={category}
                        buttonClassName="euiAccordionForm__button"
                        paddingSize="l">
                        <div css={{ display: "flex", flexWrap: "wrap" }}>
                            {categoryMap[category].flags.map(flag => (
                                <FlagCard
                                    key={flag.id}
                                    displayName={flag.display_name}
                                    points={flag.points}
                                    services={categoryMap[category].services
                                        .filter(s => s.challenge === flag.challenge)
                                        .map(s => s.name)}
                                />
                            ))}
                        </div>
                    </EuiAccordion>
                ))}
            </div>
        </>
    );
};

export default Index;
