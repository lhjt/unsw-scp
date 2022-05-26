import { Card, Text, Divider, Code } from "@geist-ui/react";
import Head from "next/head";
import EnrolmentForm from "../components/EnrolmentForm";
import { styled } from "../stitches.config";

const CentreLayout = styled("div", {
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    height: "100vh",
    flexDirection: "column",
});

export default function Page() {
    return (
        <>
            <Head>
                <title>UNSW SCP (COMP6443) - Enrolment</title>
                <meta
                    name="description"
                    content="Enrol in the UNSW Security Challenges Platform for COMP6443"
                />
            </Head>
            <CentreLayout>
                <Card width="min(600px, 90%)">
                    <Card.Content>
                        <Text b my={0}>
                            COMP6443 - Challenges Platform Enrolment
                        </Text>
                    </Card.Content>
                    <Divider h="1px" my={0} />
                    <Card.Content>
                        <Text>
                            To use the challenges platform for this course, you will have to enrol
                            yourself with the system. This involves a link being sent to your email,
                            from which you will be able to download certificates that you will
                            install to your computer.
                        </Text>
                        <Text>
                            The reason that you require these certificates is because the platform
                            relies on <strong>mTLS</strong>, which is a method of identifying
                            yourself with the server on each request without having to log in each
                            time.
                        </Text>
                        <Text>
                            Please fill out the form below to have your certificates emailed to you.
                            If you have already been sent your certificates and you have downloaded
                            them, this form will no longer work for you.
                        </Text>
                    </Card.Content>
                    <Card type="warning" marginLeft="1rem" marginRight="1rem">
                        <Card.Content>
                            <Text>
                                The email you receive will be accompanied with a password. You will
                                need this password to install the certificates.
                            </Text>
                        </Card.Content>
                    </Card>
                    <Card.Content>
                        <EnrolmentForm />
                    </Card.Content>
                </Card>
            </CentreLayout>
        </>
    );
}
