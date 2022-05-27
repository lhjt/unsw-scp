import {
    EuiBadge,
    EuiButtonEmpty,
    EuiCard,
    EuiFieldText,
    EuiGlobalToastList,
    EuiLink,
    EuiSpacer,
    EuiText,
} from "@elastic/eui";
import { Toast } from "@elastic/eui/src/components/toast/global_toast_list";
import React from "react";
import { FunctionComponent } from "react";

interface FlagCardProps {
    displayName: string;
    submissionDetails?: string;
    points: number;
    services: string[];
    flagId: string;
}

const FlagCard: FunctionComponent<FlagCardProps> = ({
    displayName,
    submissionDetails,
    points,
    services,
    flagId,
}) => {
    const [hasLoaded, setHasLoaded] = React.useState(false);
    const [submissionDets, setSubmissionDetails] = React.useState("");
    const [flagValue, setFlagValue] = React.useState("");
    const [isSubmitting, setIsSubmitting] = React.useState(false);
    const [toasts, setToasts] = React.useState<Toast[]>([]);

    const dismissToast = (toast: Toast) => {
        setToasts(toasts.filter(t => t.id !== toast.id));
    };

    React.useEffect(() => {
        if (!hasLoaded) {
            setSubmissionDetails(submissionDetails);
            setHasLoaded(true);
        }
    }, [submissionDetails, hasLoaded]);

    const handleSubmit = async () => {
        setIsSubmitting(true);

        const response = await fetch(
            `https://ctf.${
                process.env.BASE_DOMAIN ?? "local.host:8443"
            }/api/flags/${flagId}/submit`,
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify({ flag: flagValue }),
            }
        );

        if (response.status === 202) {
            setSubmissionDetails(`Submitted on ${new Date().toISOString()}`);
            setToasts([
                {
                    id: "submitted",
                    title: "Submitted",
                    color: "success",
                    text: "Flag submitted successfully",
                },
            ]);
        } else if (response.status === 400) {
            setToasts([
                {
                    id: "invalid",
                    title: "Invalid",
                    color: "warning",
                    text: "Invalid flag submitted",
                },
            ]);
        } else {
            setToasts([
                {
                    id: "error",
                    title: "Error",
                    color: "danger",
                    text: "Error submitting flag",
                },
            ]);
        }

        setIsSubmitting(false);
    };

    return (
        <EuiCard css={{ margin: "1rem", flexBasis: 400 }} textAlign="left" title={displayName}>
            <EuiGlobalToastList
                toasts={toasts}
                dismissToast={dismissToast}
                toastLifeTimeMs={10000}
            />
            <EuiBadge>{points > 1 || points == 0 ? `${points} Points` : `1 Point`}</EuiBadge>
            {submissionDets ? (
                <EuiBadge color="success">{submissionDets}</EuiBadge>
            ) : (
                <EuiBadge color="danger">Unsubmitted</EuiBadge>
            )}
            <EuiSpacer />

            <EuiText size="s">
                <h4>Related Services</h4>
                {services.map(s => (
                    <EuiCard
                        key={s}
                        onClick={() => {
                            window.open(
                                `https://${s}.ctf.${process.env.BASE_DOMAIN ?? "local.host:8443"}/`,
                                "_blank"
                            );
                        }}
                        css={{ marginBottom: "1rem", marginTop: "1rem" }}
                        title={`${s}.ctf.${process.env.BASE_DOMAIN ?? "local.host:8443"}`}>
                        <EuiLink>Visit this service</EuiLink>
                    </EuiCard>
                ))}
            </EuiText>
            <EuiSpacer />
            {!submissionDets && (
                <EuiFieldText
                    fullWidth
                    placeholder="Submit flag"
                    value={flagValue}
                    onChange={e => setFlagValue(e.target.value)}
                    disabled={isSubmitting}
                    append={
                        <EuiButtonEmpty
                            size="xs"
                            disabled={isSubmitting}
                            isLoading={isSubmitting}
                            onClick={handleSubmit}>
                            Submit
                        </EuiButtonEmpty>
                    }
                />
            )}
        </EuiCard>
    );
};

export default FlagCard;
