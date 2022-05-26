import { Button, Input, Radio, Spacer, useToasts } from "@geist-ui/react";
import { styled } from "@stitches/react";
import { FunctionComponent, useState } from "react";

const CenteredLayout = styled("div", {
    display: "flex",
    justifyContent: "center",
    alignItems: "center",
    height: "100%",
    flexDirection: "column",
});

interface EnrolmentButtonProps {}

const EnrolmentButton: FunctionComponent<EnrolmentButtonProps> = () => {
    const [isSubmitting, setSubmitting] = useState(false);
    const [radioValue, setRadioValue] = useState("1");
    const [zidValue, setZIDValue] = useState("");
    const [email, setEmail] = useState("");

    const [toast, setToasts] = useToasts();

    const isValid = () => {
        if (radioValue === "1") {
            const regex = /^z\d{6,7}$/gm;
            return regex.test(zidValue);
        } else {
            const regex = /^.+@cba\.com\.au$/gm;
            return regex.test(email);
        }
    };

    const handleSubmit = async () => {
        if (!isValid()) {
            return;
        }

        setSubmitting(true);
        try {
            let result = await fetch("/api/certificates/enrol", {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({
                    email: radioValue === "1" ? `${zidValue}@unsw.edu.au` : email,
                }),
            });

            if (result.status === 200) {
                setToasts({
                    type: "success",
                    text: "Enrolment successful! Please check your emails, including spam.",
                    delay: 10000,
                });
            } else if (result.status == 400) {
                setToasts({
                    type: "warning",
                    text: "You have already downloaded your certificates. Please contact an administrator if this is an error.",
                    delay: 10000,
                });
            } else {
                setToasts({
                    type: "error",
                    text: "An error has occurred. Please try again.",
                    delay: 10000,
                });
            }
        } catch (error) {
            setToasts({
                type: "error",
                text: "An error has occurred. Please try again.",
                delay: 10000,
            });
        } finally {
            setSubmitting(false);
        }
    };

    return (
        <CenteredLayout>
            <Radio.Group
                disabled={isSubmitting}
                value={radioValue}
                useRow
                onChange={(e) => setRadioValue(e.toString())}
            >
                <Radio value="1">
                    UNSW zID<Radio.Desc>Enrol using your UNSW zID.</Radio.Desc>
                </Radio>
                <Radio value="2">
                    Associate Email
                    <Radio.Desc>Enrol using a valid associate email.</Radio.Desc>
                </Radio>
            </Radio.Group>
            <Spacer h={2} />
            {radioValue === "1" && (
                <Input
                    label="zID"
                    placeholder="z5xxxxxx"
                    value={zidValue}
                    onChange={(e) => setZIDValue(e.target.value)}
                />
            )}
            {radioValue === "2" && (
                <Input
                    label="Email"
                    placeholder="xyz@cba.com.au"
                    value={email}
                    onChange={(e) => setEmail(e.target.value)}
                />
            )}
            <Spacer h={1} />
            <Button onClick={handleSubmit} loading={isSubmitting} disabled={!isValid()}>
                Enrol
            </Button>
        </CenteredLayout>
    );
};

export default EnrolmentButton;
