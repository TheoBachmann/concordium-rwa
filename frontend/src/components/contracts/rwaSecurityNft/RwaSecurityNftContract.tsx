import {
	Stack,
	Grid,
	Paper,
	List,
	Divider,
	ListItem,
	ListItemButton,
	ListItemText,
} from "@mui/material";
import {
	Navigate,
	Route,
	Routes,
	useLocation,
	useNavigate,
	useParams,
} from "react-router-dom";
import { ContractAddress } from "@concordium/web-sdk";
import EntrypointsList from "../../common/EntrypointsList";
import {
	ENTRYPOINTS,
	ENTRYPOINT_DISPLAY_NAMES,
} from "../../../lib/rwaSecurityNft";
import { ENTRYPOINTS_UI } from "../../../lib/rwaSecurityNftUi";
import NftTokensList from "./NftTokensList";
import { RegistryWidgetsType, UiSchema } from "@rjsf/utils";
import MetadataUrlUi from "./MetadataUrlUi";
import ContractBreadcrumb from "../common/ContractBreadcrumb";
import { ContractType } from "../ContractTypes";

const entrypoints_ui_customizations: Record<
	keyof typeof ENTRYPOINTS_UI,
	{ uiSchema?: UiSchema; uiWidgets?: RegistryWidgetsType }
> = {
	mint: {
		uiSchema: {
			tokens: {
				items: {
					metadata_url: {
						"ui:field": MetadataUrlUi,
					},
				},
			},
		},
	},
};

export default function RwaSecurityNftContract() {
	const { index, subIndex } = useParams();
	const contract = ContractAddress.create(BigInt(index!), BigInt(subIndex!));
	const navigate = useNavigate();
	const { pathname } = useLocation();
	const paths = pathname.split("/");
	const path = paths[paths.length - 1];

	return (
		<Stack>
			<ContractBreadcrumb
				contractType={ContractType.RwaSecurityNft}
				index={index!}
				subIndex={subIndex!}
				entrypointDisplayNames={ENTRYPOINT_DISPLAY_NAMES}
				path={path}
			/>
			<Grid container spacing={1}>
				<Grid item xs={12} md={9}>
					<Paper variant="outlined" sx={{ border: 0 }}>
						<Routes>
							<Route
								path="tokens"
								element={<NftTokensList contract={contract} />}
							/>
							{Object.keys(ENTRYPOINTS).map((entrypoint) => (
								<Route
									key={entrypoint}
									path={entrypoint}
									element={ENTRYPOINTS_UI[entrypoint]({
										contract,
										uiSchema:
											entrypoints_ui_customizations[entrypoint]?.uiSchema,
										uiWidgets:
											entrypoints_ui_customizations[entrypoint]?.uiWidgets,
									})}
								/>
							))}
							<Route path="*" element={<Navigate to="tokens" replace />} />
						</Routes>
					</Paper>
				</Grid>
				<Grid item xs={12} md={3}>
					<Paper>
						<List>
							<ListItem disablePadding>
								<ListItemButton
									onClick={() => navigate("tokens")}
									selected={path === "tokens"}
								>
									<ListItemText primary="Tokens" />
								</ListItemButton>
							</ListItem>
						</List>
						<Divider />
						<EntrypointsList
							entrypoints={ENTRYPOINTS}
							entrypointDisplayNames={ENTRYPOINT_DISPLAY_NAMES}
							selectedPath={path}
						/>
					</Paper>
				</Grid>
			</Grid>
		</Stack>
	);
}
