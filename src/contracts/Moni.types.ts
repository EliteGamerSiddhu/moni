export type Addr = string;
export type Uint128 = string;
export interface InstantiateMsg {
  cw20_address: Addr;
  extension?: Empty | null;
  max_tokens: number;
  name: string;
  owner: Addr;
  symbol: string;
  token_code_id: number;
  token_uri: string;
  unit_price: Uint128;
}
export interface Empty {}
export type ExecuteMsg = {
  receive: Cw20ReceiveMsg;
};
export type Binary = string;
export interface Cw20ReceiveMsg {
  amount: Uint128;
  msg: Binary;
  sender: string;
}
export type QueryMsg = {
  get_config: {};
};