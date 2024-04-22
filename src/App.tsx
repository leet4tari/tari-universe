import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { useEffect, useState } from "react";
import {
  TariPermissions,
  WalletDaemonParameters,
  WalletDaemonTariProvider,
} from "./provider";
import {
  TariPermissionAccountInfo,
  TariPermissionKeyList,
  TariPermissionSubstatesRead,
  TariPermissionTransactionSend,
} from "./provider/permissions";
import {
  Box,
  Button,
  Paper,
  Stack,
  TextField,
  Typography,
} from "@mui/material";

let permissions = new TariPermissions();
permissions.addPermission(new TariPermissionKeyList());
permissions.addPermission(new TariPermissionAccountInfo());
permissions.addPermission(new TariPermissionTransactionSend());
permissions.addPermission(new TariPermissionSubstatesRead());
let optionalPermissions = new TariPermissions();
const params: WalletDaemonParameters = {
  permissions,
  optionalPermissions,
};
const provider = await WalletDaemonTariProvider.build(params);

function PrettyJson({ value }: any) {
  return <pre>{JSON.stringify(value, null, 2)}</pre>;
}

function AccountTest() {
  const [accountData, setAccountData] = useState({});

  async function getAccountClick() {
    const res = await provider.getAccount();
    setAccountData(res);
  }

  return (
    <Paper
      variant="outlined"
      elevation={0}
      sx={{ mty: 4, padding: 3, borderRadius: 4 }}
    >
      <Stack direction="column" spacing={2}>
        <Button
          variant="contained"
          sx={{ width: "50%" }}
          onClick={async () => {
            await getAccountClick();
          }}
        >
          Get Account Data
        </Button>
        <Typography>Result: </Typography>
        <PrettyJson value={{ accountData }}></PrettyJson>
      </Stack>
    </Paper>
  );
}

function SubstateTest() {
  const [address, setAddress] = useState("");
  const [substate, setSubstate] = useState<{}>({});

  const handleAddressChange = async (event: any) => {
    setAddress(event.target.value);
  };

  async function getSubstateClick() {
    const res = await provider.getSubstate(address);
    setSubstate(res as {});
  }

  return (
    <Paper
      variant="outlined"
      elevation={0}
      sx={{ mty: 4, padding: 3, borderRadius: 4 }}
    >
      <Stack direction="column" spacing={2}>
        <Typography>
          This test gets the substate content of a substate address
        </Typography>
        <TextField
          sx={{ width: "100%" }}
          id="input-url"
          value={address}
          onChange={handleAddressChange}
          placeholder="Substate address ('component_XXXXX', 'resource_XXXX', etc.)"
          InputProps={{
            sx: { borderRadius: 4, mt: 1 },
          }}
        ></TextField>
        <Button
          variant="contained"
          sx={{ width: "50%" }}
          onClick={async () => {
            await getSubstateClick();
          }}
        >
          Get Substate
        </Button>
        <Typography>Result: </Typography>
        <PrettyJson value={{ substate }}></PrettyJson>
      </Stack>
    </Paper>
  );
}

function App() {
  const [balances, setBalances] = useState({});
  async function start_wallet_daemon() {
    await invoke("wallet_daemon", {});
  }

  async function get_permission_token() {
    await invoke("get_permission_token", {});
  }

  async function get_free_coins() {
    await invoke("get_free_coins", {});
  }

  async function get_balances() {
    setBalances(await invoke("get_balances", {}));
  }

  useEffect(() => {
    const handleMessage = async (event: any) => {
      const { methodName, args } = event.data;
      const result = await provider.runOne(methodName, args);
      event.source.postMessage({ id: event.id, result }, event.origin);
    };

    window.addEventListener("message", handleMessage, false);

    return () => {
      window.removeEventListener("message", handleMessage);
    };
  }, []);

  return (
    <div className="container">
      <h1>Tauri wallet daemon</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          start_wallet_daemon();
        }}
      >
        <button type="submit">Start wallet daemon</button>
      </form>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          get_permission_token();
        }}
      >
        <button type="submit">Get permission token</button>
      </form>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          get_free_coins();
        }}
      >
        <button type="submit">Get free coins</button>
      </form>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          get_balances();
        }}
      >
        <button type="submit">Get balances</button>
      </form>
      <Typography textAlign="center">
        balances: {JSON.stringify(balances)}
      </Typography>
      <Box>
        <iframe src="http://localhost:3001/" width="100%" height="500"></iframe>
      </Box>
      <AccountTest />
      <SubstateTest />
    </div>
  );
}

export default App;
