import {
	Button,
	Stack,
	Step,
	StepLabel,
	Stepper,
	Typography,
} from "@mui/material";
import ErrorDisplay from "../common/ErrorDisplay";
import { useEffect, useState } from "react";
import { useVerifierApi } from "../VerifierApiProvider";
import { AccountAddress, IdStatement } from "@concordium/web-sdk";
import { WalletApi } from "@concordium/browser-wallet-api-helpers";
import SendTransactionButton from "../common/SendTransactionButton";

export default function Registration(props: {
	wallet: WalletApi;
	currentAccount: AccountAddress.Type;
}) {
	const { wallet: wallet, currentAccount } = props;
	const [activeStep, setActiveStep] = useState(0);
	const [challenge, setChallenge] = useState("");
	const [statement, setStatement] = useState<IdStatement>([]);
	const [error, setError] = useState("");

	const { provider: api } = useVerifierApi();

	useEffect(() => {
		setError("");
		setStatement([]);
		setChallenge("");
		setActiveStep(0);
	}, [currentAccount]);

	const generateChallenge = () => {
		api.default
			.postVerifierGenerateChallenge({
				requestBody: {
					account: currentAccount!.address,
				},
			})
			.then((response) => {
				setChallenge(response.challenge);
				setStatement(response.statement);
				setActiveStep(1);
			})
			.catch((e) => {
				setError(e.message);
			});
	};

	const registerIdentity = () => {
		return wallet
			.requestIdProof(currentAccount.address, statement, challenge)
			.then((proof) =>
				api.default.postVerifierRegisterIdentity({
					requestBody: {
						account: currentAccount.address,
						proof: {
							credential: proof?.credential,
							proof: JSON.stringify(proof?.proof),
						},
					},
				}),
			)
			.then((res) => res.txn_hash);
	};

	const GenerateChallengeStep = () => {
		return (
			<>
				<Typography variant="h5">Generate Challenge</Typography>
				<Button onClick={() => generateChallenge()}>Generate Challenge</Button>
			</>
		);
	};

	const RegisterIdentityStep = () => {
		return (
			<>
				<Typography variant="h5">Register Identity</Typography>
				<SendTransactionButton
					disabled={!challenge || !statement}
					onClick={registerIdentity}
					onDone={() => setActiveStep(0)}
				>
					Register Identity
				</SendTransactionButton>
			</>
		);
	};

	return (
		<Stack spacing={2} m={2}>
			<Stepper activeStep={activeStep}>
				<Step>
					<StepLabel>Generate Challenge</StepLabel>
				</Step>
				<Step>
					<StepLabel>Register Identity</StepLabel>
				</Step>
			</Stepper>
			<Stack spacing={2}>
				{
					{
						0: GenerateChallengeStep(),
						1: RegisterIdentityStep(),
					}[activeStep]
				}
				{error && <ErrorDisplay text={error} />}
			</Stack>
		</Stack>
	);
}
