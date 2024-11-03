// This file is part of kvp.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::*;
use hex_literal::hex;
use pallet_society::migrations::from_raw_past_payouts;

parameter_types! {
	pub PastPayouts: Vec<(AccountId, Balance)> = from_raw_past_payouts::<Runtime, ()>(vec![
		(hex!["04152389a92e4356ed03ed30625afba062b9c4496116cba888e89f347834a31e"], 89000000000000u128),
		(hex!["0483de2b96cd3756f05301eb1bca875166ae5be67a81f86a1dad152bccad6909"], 54666600000000u128),
		(hex!["048a62b89589f7573c5f80584de850b9c8bcd4cb7a8766948d3db713d8a57204"], 83000000000000u128),
		(hex!["060e1eacd9f4460ae43b28d39e472078784482139e483307436344019a48f26e"], 15513300000000u128),
		(hex!["0ab0b61984dfcfe2fcb82147f0a2f00f992fa8a6b5ee81490387f8210a1ab678"], 6766666600u128),
		(hex!["0c6e31c65ee1e82129879a145eb0c0d4de45e60c3dcab1d2219628cd70673a6b"], 69000000000000u128),
		(hex!["10e9e630de91310a60b0bc7a4a2303a60b703afde9239750921514deb2655a0d"], 54333200000000u128),
		(hex!["162eb21266fef25e25eb8286994264450b8c80ccf2911967cc42ee4cc55c2061"], 14223300000000u128),
		(hex!["183f8c4d5084d96816ae7d82d44373b99bb134a16845d1589df4467671e3b56a"], 23333300000000u128),
		(hex!["18b2624ddf0eca6c9698496801dcda614580367a4d54833fe75bbf54a9a09966"], 52999900000000u128),
		(hex!["18b44d4c64c3aa3fb20d88b016a412d8f27000912fa7b350ed270ac8f55b3a27"], 5135000000000u128),
		(hex!["1aa23df47ab442adc15f6c923cfd0f6c4255b2cacc19538028685e37410a691f"], 54000000000000u128),
		(hex!["1ce0527c225ce227c1cceed0317eae0817bd600be3aa87489a704678c95a5451"], 41166600000000u128),
		(hex!["1e20318bd5c0bc44576955b7205354b8ed9c8e4a783ba9da70d5cee0e11b4740"], 11000000000000u128),
		(hex!["1e6fa8ff46b01fe3b8c52ea50a5ab61313ffac31393a28f621625840a6b3776d"], 11556600000000u128),
		(hex!["20243c6b74a5f83b89ec202a337bd06ebc985fe7b1557184bfd012dc9bc97873"], 13000000000000u128),
		(hex!["22cabf36e0954f0c013af666e27e46cb9a41f7a0db46ccec245f43b5ba438452"], 31000000000000u128),
		(hex!["24172a563943291c97d252def71e17abf467a1626bca358728a90a82b3de3118"], 200000000000000u128),
		(hex!["247f22a3073d04a85ab417505bb7667a5941ed74cfa596dfc7402813198a0560"], 68999000000000u128),
		(hex!["28250c422851313e923c89cc0a41fec2af80cc124290343ae250f77ff60d767d"], 6993300000000u128),
		(hex!["2c709012f807af8fc3f0d2abb0c51ca9a88d4ef24d1a092bf89dacf5ce63ea1d"], 333333333332u128),
		(hex!["305b27652a4dc8038c587df4cf1d7758b413fe18a97b3a3a5c7d609f50092e11"], 39000000000000u128),
		(hex!["3063796fb70f0bcde597bb7ed4d50f6ec7755686c894c7dbaa1ea2e33103876e"], 11650000000000u128),
		(hex!["325c848b9000d5430844bd5486d34d844acd89e11964ed1b535bf45557e1c87f"], 10666600000000u128),
		(hex!["3a632ab63a0ec92f1b67af756a49c8849f5722a345e54ef51cb294bef569070b"], 14993300000000u128),
		(hex!["3c97e1879015dfb64a367e0f0eb32f16968fb6be6a48da7de7776bc8953c854d"], 18149900000000u128),
		(hex!["3e8eb90cd422b7d72f166f135ed3db2137d0d9216e14cfdbd0ca75d2a18e2235"], 83000000000000u128),
		(hex!["3ea89a71ac11ea023bf42d9d6215a9d8c775e626a2ee38d574b297f42c58a622"], 15056500000000u128),
		(hex!["402d95c5f6d37ab03ecbec4628d290b2cc7571ff790ee5757b21bbcba4108924"], 12666600000000u128),
		(hex!["40300fe00bcfb90f3ded42dc082b775dd9a8a8ea491261a262a78155a069a268"], 112000000000000u128),
		(hex!["406855a4bb07710384d3876cf37cacbd66f2f1dab2346ebea130b00f63d0d317"], 83500000000000u128),
		(hex!["40c6e021d4d80b9b38d850b1c5334ea88b2bcc148d07e83f5be1b45b8ceb3740"], 9893300000000u128),
		(hex!["40f8a816b07bd23e166d2e479cfd5a3e5118f3937230ca5088eff9e33c84b552"], 12790000000000u128),
		(hex!["422286e0da2fb826f04e46d2f6ef1319a43412261d0f8d91274a81eee21af24e"], 69000000000000u128),
		(hex!["484a648ebe737d7dbee39d8e169d5ded94d29f70100e4d3ee87162c4a8bbdf73"], 2483300000000u128),
		(hex!["4c4a2f66cd9f5000968f0913f01ac1181ccd2db137d6af152252ee3de689450b"], 2000000000000u128),
		(hex!["505eb7820f60d0949697617b2f3366bd616d8c7e96724aa681e0113f6bf45c46"], 898900000000u128),
		(hex!["569f5ab70b93bb40ecb5a8888bbbc781c785ac3709863e9866422f0fd62f2477"], 8000000000000u128),
		(hex!["5c1be3d517926a6c194d42131d996140f3e8d7398764423cab176341b882ee7b"], 8490000000000u128),
		(hex!["5c32313c22eebc15ecf28ebb75ed8e264d53e50429a5ce09ed3a86e72732c56d"], 5000000000000u128),
		(hex!["5c436629095023be4d2ed2120002497bc18295fb485a11e83e529e617412626f"], 6999900000000u128),
		(hex!["5c81f3afd924f4cdd7c151d539c7abc3cd3de33eebd59403b81b568b8efa2d3b"], 15000000000000u128),
		(hex!["602d798e4d6f076cb28719b4bc757645ff0894a591173bd923bc8cb631fdeb0c"], 201660000000000u128),
		(hex!["621014fccda62dbd21d32b3628691f68cecafa887a62d641ba8876e3e7e4c068"], 8000000000000u128),
		(hex!["6613962cf897114a56ba84bbad47f52c46dc56960aae8ccfc71805cfc3fced19"], 32666600000000u128),
		(hex!["662ef2fa0d3a90f1f9691d05daee187d35fb17482cc72b3e03922358d0bdcc6f"], 10000000000000u128),
		(hex!["687b7e0289f5d116a2b68cf9d0496f62de37e579ea777ce39d81471c09ec142b"], 275000000000000u128),
		(hex!["688de40f61eb6ecc19b4c3702267c0bbf052ed9ee843ff6d346d765f89ed6067"], 7000000000000u128),
		(hex!["68fae6be10c90d572388d42129e074005005baf68d116a993073c5648ec78865"], 6356600000000u128),
		(hex!["6a8cafed3a670189545d5b242aec4d52bb4fe90f5af2f5984d8a14eb44713a70"], 12780000000000u128),
		(hex!["6c64f0ac3b73174aa0b0cd935b5576611e405c8485ef13a0be8bf2ea3a48da6f"], 8000000000000u128),
		(hex!["6eb31a06ff9d943b174f683f8327e3b4847a02e197e951f01ac7759b4c102f10"], 52333300000000u128),
		(hex!["6ed537e76f1ef68764d7544cd7a8be19cbaba2ef8af181090d281d80105fd963"], 175000000000000u128),
		(hex!["7241b3a590243df346a79a2d0ebf79dc990f07c1c499145d0424f3769ca4c826"], 81000000000000u128),
		(hex!["728ea7cd638962b92ec6405e7b5572b67cfbc96c7c2fc7becf4cddb22b50b02c"], 11890000000000u128),
		(hex!["7a91a646fd4d7592aabed6ad7f3c1d9f12371f200ae3d644454c80272b8e8c14"], 44000000000000u128),
		(hex!["7c9ae158bf660dbd429592d055efe4897fad8a07c4ed61accd1861380c5c3843"], 24666600000000u128),
		(hex!["7cb95b196a81bb0b7952e94ff0624b9b1429e81bee0f03c8bf2f3cea5213c611"], 5805000000000u128),
		(hex!["7ec0c61a682519e78e65026c51ceea52273870636814605a33518f02ad543317"], 6969600000000u128),
		(hex!["802c32932fca84ba9c80d57e4b0ebccc8404ee75a346ae0b50a17666bb38c01c"], 4986600000000u128),
		(hex!["806369b4f04792b7bd1a0d8586b7aa528591ba732362e3fb3d52d7b01e741b18"], 8660866667000u128),
		(hex!["80a551df4b4b67586a512356fc0e513d641e39f62172d185465dfee61e67143a"], 15160000000000u128),
		(hex!["821e24cd21f1da627bea7b7077d591b9c8a48f93dd87eb7f3f1ac4ec512f7b5f"], 7500000000000u128),
		(hex!["84c62d27805ac9c7a62086e31dfae23703ac9dfb37fbd31bec95aa611c5d2c33"], 999666600000000u128),
		(hex!["865facd74193d96f1b35a702efe05116e7be752d77f46c7fe4720905728ccb36"], 6993300000000u128),
		(hex!["88077737732044369dc52dd7f7bf400cfb493a219f013fbfdd46f7cd52673d6f"], 67666600000000u128),
		(hex!["8865278958eaaba42406d1ae16545267c944113c216fffc386edb4d6a8cffd36"], 5000000000000u128),
		(hex!["8ad06fa44a5669702a29b394424560714a1af90ad9efb57f3864b93b1ff7961c"], 100000000000u128),
		(hex!["8e7215d5218f170d0865fcbe16b2ceb752db7e7bfce3f3d487ecb60e776a2c36"], 39333200000000u128),
		(hex!["8ef7167c4d50be846c6a03591e13005fa68ef858d87321bb79428b121e105a11"], 9166600000000u128),
		(hex!["90bc6fc1133c3df447222e89b5ceaa028b69348ee381385239377b31df275248"], 15000000000000u128),
		(hex!["90da58a51d922ae69f27d8020d52e2fc71a5e5af1b63571bae81ddb87f8ff424"], 6696900000000u128),
		(hex!["92af8236baeef25f6e85ed30d85b758f5604c5c4c4c3637657fea1946a3da61b"], 13000000000000u128),
		(hex!["969fe4cba88544e8d3d71f31790cc1d377cf85a89e1b3d03e7a8b932aed1d312"], 84000000000000u128),
		(hex!["96be4635aeb775be58f3c0843bd8bc1832d257e56905525dcb3d38126d8f855e"], 12166600000000u128),
		(hex!["9822df7da6d3c119f5118587b29eaa30aaf00839e2a03161c2df4222a0d8744e"], 8999900000000u128),
		(hex!["9cb774a6051717a844657f0c037a93173f70afc000babd2f7f0be7d1e8475436"], 9000000000000u128),
		(hex!["9eb27cde65b09610cbe8a3d3b82c6730f5a5dd51aeb082fdd74236c4765a040a"], 8890000000000u128),
		(hex!["a0c6e8d74992d1ea3e43081624b2cddbd98b50d7a75998662ce6adbb2290aa64"], 175000000000000u128),
		(hex!["a2b8bfb3c0c1f04346134e7b27cb5b63de8a7af0d57c502d09c05ba7b3dd1e28"], 9326600000000u128),
		(hex!["a415a980463876c54b503c358613b5c02d8ac9781c13378797c54ab37fc07c05"], 8666600000000u128),
		(hex!["a48539457aa2e54048493ccaf980be18253d8cabd6eecd295e6b62e6a357352f"], 8879900000000u128),
		(hex!["a60b82ce304c28aad744e2a95924b3e1e560803a75e5a39fc91300556e9a9538"], 83499000000000u128),
		(hex!["a673009c77c4734fbc09f3bf505e3414282f714e89688ca3fa9292921cb7e51d"], 93000000000000u128),
		(hex!["a69f19b16dfdfb01b1d480c59512b0d589e600538cd9102de9619ef419211f1b"], 11993300000000u128),
		(hex!["aa17e09d3e9685a53d52d5123cc7f4f6f9fc7f3ab34f268a6f8860de68f2b612"], 15000000000000u128),
		(hex!["aa893ab408a0c0b8bb175ed7fa2b042e0fd30915e6ab8f66a9ac524e552d167f"], 11480000000000u128),
		(hex!["accd5106f4794d9052617617a993556b15c6e62859282bad7edb24592d728a69"], 94999000000000u128),
		(hex!["ae4bada5af908d3bddd0cb7e250d38da99cf9d6508a6b1118a89dc59fb372a4a"], 17000000000000u128),
		(hex!["b2012e8078883fdd9693d75c90bb669834f7b2c302def049e6eb486e56dc7365"], 13500000000000u128),
		(hex!["b2fa9763f56890cabe29a1ea971851a3234aecd03584c44f822e036fbd9c5156"], 8554600000000u128),
		(hex!["b8a750597d770430e4f9771829cf964825ea6750a6229deb06dc515371c7033f"], 96600000000u128),
		(hex!["ba8a15ff06ff808a77b93ad335884ac66e3744a5cea908f8b8a865d98a934541"], 21333300000000u128),
		(hex!["baf98d06056833e2e887c85c938aeb1c31dff74a91d35d9863327abc8ec93f4a"], 7500000000000u128),
		(hex!["c2487c00d0e309fdb96d1e0ed7bb2da173d777b3ae26b25c1369992add25972d"], 21000000000000u128),
		(hex!["c2fc6a3cc910e05c508c8b252f32c6df760858d12c6a636b256135e966edcf7e"], 9715500000000u128),
		(hex!["c443922426297f4ded0e635f0a92b8e31823b2e6893398b1e8351c34a728fe34"], 6493300000000u128),
		(hex!["c4f4760edf86fffa05380f31bf047c72837cccfc684d9dd9b1d7fa4e3503ce51"], 14500000000000u128),
		(hex!["c676a26c89e3be0451afa1126516b8f88b3da99511aba215ec99707ac5f08e3c"], 14833300000000u128),
		(hex!["d25af2fedd4eb672f218932fde44f97f10c1d7788efd0079957ffad4f186ae78"], 100000000000000u128),
		(hex!["d4610b986b4e4cb505ce0003142df98f803b7c3413acc9f7805992dc2a00483c"], 110000000000000u128),
		(hex!["d80cb19a68fb4ae325cd0209e8563cf3d5ff0368e2ede5530940f29371a02a25"], 19000000000000u128),
		(hex!["dc8f45881886ba4d2e2409ea49661b14a29a72e64d7a59d98465a9cee8084107"], 34666600000000u128),
		(hex!["de3898d3824e41435b6519f2095d25af51954af6be4a946d5d2df46df3ea264f"], 110000000000000u128),
		(hex!["e2094a5bcc479f2e6c83bfdbc88fe3658b817522ba1fb240f804b131ffa81600"], 41000000000000u128),
		(hex!["e27bc8259449251380d0b6c848cd607b12a09b0fa8fd8875ae3f6eea70c3fd69"], 1993300000000u128),
		(hex!["e45c09f0387a72f3a1eeb9e1a8f23feb738bc48d56d99ab88c8a910807c48a0c"], 18646600000000u128),
		(hex!["e62321ed84ec54791122f2ec72e9e36d3cb336ed358d6848a65b8410b405650a"], 9216100000000u128),
		(hex!["e80d8a511c20f08d8abbd69f1258fa27c28181ffa8fbee0989f706e7b6c48b21"], 9000000000000u128),
		(hex!["ea53405eb9054ddf0b6b82de940a4646c70cb815ba4c4616394fe0488030bb32"], 72333300000000u128),
		(hex!["eedfb6337bed7b15d7a0338820e8a4981d96fe7284e444885c7f478fe649012f"], 14566600000000u128),
		(hex!["f20f603b0314b04a4c3b295cbfc7b53c11370cc0349ddb3cd32c91c5b416fc51"], 7493300000000u128),
		(hex!["f4fa6e013f0a33b809b8c1dc8d73c1461407a474106f5def66c109a3d7c4f556"], 56666600000000u128),
		(hex!["faaede0e8cfb95d55e325e29a4737decb4a20960a525384003248c5610405b7d"], 1893300000000u128),
		(hex!["fae2f8b2e08e32c333e0332b4254119241c2f15421846b76b693eba714b3e571"], 20333300000000u128),
		(hex!["44152e29de73d969a8d5bc6d0b3497f31ee7e7f6e01722a5a91fccadd6bcce76"], 14900000000000u128),
		(hex!["4adb5df8ae7001c508d3e630deda167bce1760aa0e7c8544a1e3b70358ac3b45"], 15900000000000u128),
		(hex!["f04581f47bc54daf59437fe8a8e7e76ab3f034d8b30f3629652a6a013f7e0b38"], 13986600000000u128),
		(hex!["9c107fbcac10f60dc1910e27210283c39f8d5951816f8d7c8f5f96d0c71dbb29"], 26666666400u128),
	].into_iter());
}

#[test]
fn check_addresses() {
	use sp_core::{
		crypto::{AccountId32, Ss58Codec},
		hexdisplay::HexDisplay,
	};

	let payouts: Vec<(&'static str, f64)> = vec![
		("CffzJo8UPWwvwPF73VcbEv4jSG4ckvGwNePL9V52hYh743X", 89f64),
		("CgEt8AwW9SThQXpLBAZy3MpKgNG7ZHaEDGeV5MLqHVPVoJg", 54.6666f64),
		("CgGpRVgE8WXd2hjc3GBVxGpnG9KpjkvEGNkM6AAaCBmhQY9", 83f64),
		("CiFzvmP1wyXB8nUHh6rA1j7tFQFBx2akg57TwdTGjRAXicm", 15.5133f64),
		("CpLVRWcUd7PxDPSmtXwQC3628dwEYiMd3GbqZTnqvphMN3B", 0.0067666666f64),
		("CrcpvEZP2Z82iYFUPConih1t46VFwttrX7Tv19NW6hoCzKG", 69f64),
		("CxVmQoEyZKimVp3eMgtNHSwiFrEU3gwF1NGNPBo2gUNSr1g", 54.3332f64),
		("D5QSdd589pFWeJm9bz1j3RTyy14ejKpnUKyEa4GBW9kYkVC", 14.2233f64),
		("D87YYjk4agajQXHr1VyiW1qpzfm5QV4L8bk7XduhimVVS7H", 23.3333f64),
		("D8hf8DjZ2eb8X6cyNAYx14fRa7hVUqB52ejx5YNc2ed81gC", 52.9999f64),
		("D8iEArcApNbkH9BxEvc2AfLLWMD1sWS9F2XdRXyY9J6QDpi", 5.135f64),
		("DBExZRq4qoK8xiZaHTR7iP9tWmQ8FL9JSQG8DLyLL8yV2J8", 54f64),
		("DEBVjNhPic2eVBw2y42xJpTJfCSvDXXA6ibs7MzpXp2s2hj", 41.1666f64),
		("DFpWz9jGga5ZKcRghQGchhMhSHURUcUc2TaT7dMB4pQhkeB", 11f64),
		("DGE8ATd2NaitqX4jdvZNXFNMmY9Qui6swnfoheCiz7efWGG", 11.5566f64),
		("DJTpC2pbDJeoJ2CTHSQBurbkbd9ZgkD2WYNDtCmJhem3swh", 13f64),
		("DMwNfM1mwrraAoSG3LSnZcwGzuUcKipqYmRsFeCuvTvrdgL", 31f64),
		("DPe86fQfixDTfejAiEJbRt2mvbtcGkWdGXsyCFDu37iKYVu", 200f64),
		("DQB1TYcr7dw4UsHaNbNhM2jtyYaoQQrBoWS7jLb95D39XGb", 68.999f64),
		("DUxSQ29BxeZWDXin4jZN7ogArgxJDohyeuKsqY83WqPhT4h", 6.9933f64),
		("DaViizibrmJZwyUchRMRebv5YMXadS4PbYBLuZhbfMTtxPm", 0f64),
		("Dab4bfYTZRUDMWjYAUQuFbDreQ9mt7nULWu3Dw7jodbzVe9", 0.333333333332f64),
		("DfitqjAjNxJykJYaigWQmSRw7T847hfytxwVRuAwscdtYUh", 39f64),
		("DfmNCWtsVSG9D8KWazZ84VdkSVgxot6989W1qL7bHEAP7SF", 11.65f64),
		("DiMPtqB6HeYpYkjJJKNR6btJjmUNeG8yyhSg1N8UL3st5Um", 10.6666f64),
		("Dikw9VJqJ4fJFcXuKaSqu3eSwBQM6zC8ja9rdAP3RbfeK1Y", 0f64),
		("Dtskg3rQsxSpxxBG4pJYjTujm9fo48Q7qnEYtBfJ6UDqtSS", 14.9933f64),
		("DwmWULE1g84ZMYyM4du8SmcQCkuz7VL7C6GmaWKoRXSG6vJ", 18.1499f64),
		("DzLtEdo7ScPi1o3izPETynSm9kdXJF8H1Y68b3eFbsQUgm6", 83f64),
		("DzUa9PynyTKBEJ9Yjk27o44aav9XaJG8p57YHKVNaorxbZu", 15.0565f64),
		("E2U89NfSnVCmY3hBQHEdJ9KV6shd6B8h4EWkNLcSJCPRAk8", 12.6666f64),
		("E2UrpeDCGs2mb9SZLzAk8E89yWaZncAvjFD5VnbYbDp44Xo", 112f64),
		("E2maNj3d7YZZhpeUwCGGeThD7JX2zk1W8AUQ1Rqb2K9oDBR", 83.5f64),
		("E3FfH6nbxCwQ9bg3oHAmBFpS3qEtWLbQ4QV8XUqn6qQaaab", 9.8933f64),
		("E3WSzAdgZtu3o8NjTJdwweJjpmSGwcf42pZAFDR7CsNTrrr", 12.79f64),
		("E52wAv1fTdLGUNR4CNSbeCmj1yNUeSFnj1CPKbRzTubt3YU", 69f64),
		("ED74i7eA79DtKFu5WXbcLXLWT2E1XSDEdJSAEkU3ePN7LE4", 2.4833f64),
		("EDQWMYr6a9aLjTbsMtKFAcoUsVA6qvm66RJsyJsNgbpxCFx", 0f64),
		("EJ4FuvmVpU7Ri2GRMtQVhhfHsHzGLYngbKJzJ58RkmRrzJm", 0f64),
		("EJMCExEPNyuq6EofbvwD2ErZKwuZCJVTFhiDcDwuW12c187", 2f64),
		("EPhV6vifCet2sJPPCNGnUZdRhsQpfcsbxRvDkjsMPPCCvwR", 0.8989f64),
		("EQ12pCgs4H3XHgDTc5n44xfXo6WNNkAg33b4aDFv3eUSgZd", 0f64),
		("EXtySo37DkJUdQb5425KGzUCG8ecYL4nVkcNB5XZpDQPH73", 8f64),
		("Ef6D2jpq92FoX8wSKihodFCg8juMjFu9CBnMwwBQDbYi4XU", 8.49f64),
		("EfCqHrWEwRnRq2ekmTmWvKzYwKtcgD4uaJAm3T92wx3ekUx", 5f64),
		("EfHwkqXCDup8Hi7DVWsPTDQoGiaW55HXgbQV3XzVSCEu1ii", 6.9999f64),
		("EfcXWwDMt9UoYhj6KRfEA4NLccsZK2eQ2AFpGw4EGatNwcj", 15f64),
		("EkRd7vCKiDZi6BM6tHKs5YAeGmE3MmQgChrrbtkRkf6SBwW", 201.66f64),
		("EntzFKky1rX9oYWzEiv4K4D997XGtCH9BHCmairrXhrW2gb", 8f64),
		("EtADjuauj4ETanscktz4jpejZdkCMVPnRxKos6AwgpzLdHM", 32.6666f64),
		("EtJMBiUVsHm3bwfMhWksQ14mv1P69BHjp7ELYDWdqXynR6K", 10f64),
		("EwKBYgaaELEEB5Vm9QAgArinAtps3dRUmb9bwBVTg5HqvxJ", 275f64),
		("EwQeYkCGQtBHMgKvNZ7NZ9Dy9tqKJDqN3DknJm5XF8czbYB", 7f64),
		("Ewy2kju7jdqBFbp95Nw1go31KyiMHS9rRVaevGkM62F1btd", 6.3566f64),
		("Ez2PN6BKn31byeQTrTBqP6MAv4WF7eKF3J8UXXxaaAd1Kuv", 12.78f64),
		("F2Sg6L8dTASXBz4rn9Rv4xsfaCKPn21pDzAX9HSAR8krMzR", 8f64),
		("F5TzLdntQpfz5RXcqCqJM4NKrjS7jVnAcNRKhu5FpLLWm2i", 52.3333f64),
		("F5e8A2i3XZvA5jZTYmwcUiHzVsoTAhJDgMxZEnsFe8dq2VA", 175f64),
		("FA8VcXg8Yzg9RvVrTdhcKk5JLSH1hcwrS9fr7pjFraCRWcm", 81f64),
		("FAXMVgYUWh6s2SFaX9h33mrjXXeq8HTcNwFHEy9D1eimrLK", 11.89f64),
		("FM2dJMRnBkbJDbdtqhRpLfgRbwJBkiw6iKT8u6UJ7pojypt", 44f64),
		("FPhTtxHSTKZM4fTndfgBSSXyTpdMkpEnqojAvGTS1c8hv9S", 24.6666f64),
		("FPrWyj8aDy3dofjmXZE5mx4o6Kf82MnEGGpjVEHjH7khReo", 5.805f64),
		("FSWpLozBJnXdVhE8t7LXYEpBybwse3S795WYFKiobhZWKVE", 6.9696f64),
		("FUNmzDNdzWtXK51EZmwbDJXQALhcYGCQUeo8E4rtcx9QBX7", 4.9866f64),
		("FUfBKr2pDxKrxmExGp4hjU6St4BDgffzKcyAqv6pruGnez1", 8.660866667f64),
		("FUzksiAhxzSvvPqiYvUEwK32rFf6Fmyug9GZEdbMaoLmwh5", 15.16f64),
		("FWvhQBV91wrvaqWiqDZfq4QPYYPFS28UV8zNgeJYyUexxXm", 7.5f64),
		("FaQi6AhM49SdjBd7oZJftAPE6tt9mrH8AdySztpYUU1B6Vq", 999.6666f64),
		("FcWMb9VtQutzQ5hxguyQrWKtN28zaXKuXLbL3D9oX6AJbiC", 6.9933f64),
		("FcxNWVy5RESDsErjwyZmPCW6Z8Y3fbfLzmou34YZTrbcraL", 0f64),
		("FegFL7hjGtjwRWvDMabMuPFGTzToioEWu4BLxjRxYPfvCpn", 67.6666f64),
		("FfA5YrMeaPzBYB4rE4JBuauXua6BEAQoJPTd8dRCLCdRdhM", 5f64),
		("FiL3XNxVpx5Cgnh4WPpPWms5Ed9tGqA44YEtHVDyPacHVGt", 0.1f64),
		("Fo6D1N9EjneyZYYPGxiW1WNKsj1S9Gx1nxcrMCkKGQ1bwDM", 39.3332f64),
		("Fomib3HNL24Cv6CbLJUf59yDF5shGFyDg8Wx7eGtmDdwrRF", 9.1666f64),
		("Fr6PbzHWKrTvmFJmiYy7MD1iaUpTKHN91VS5iQZZgo7BWCw", 15f64),
		("FrFGwHJKqLTyGDwqWfLgGYFDkjP12yAATQuh4oNpZMhpFzM", 6.6969f64),
		("Ft2cSCw4V47d2S7V9nN2S6V5ByGmAnkfbkFbWUVuHVuaMvW", 0f64),
		("FtG8FbxJXfDj6fcGp3eTL9tKJzkfqT9k8PREKXDgKmbnSDR", 0f64),
		("FteeR6d11cvmoRUmvoBeYYHNpMd5onPnpGy5PUcR6Vo7dJX", 13f64),
		("FypCVBt2MU61ZVL3N6mePXaUXrFX9nvGK6h5yqR89jc6JeW", 84f64),
		("FyyDvxKBmKr81HwQWtbugiFRiu2P1JpSqBDid56AfFevuTV", 12.1666f64),
		("G1o9z4HVf2pW7bbyKKNRP9MnmGEN4n7Ada3yJRpNzaiPoKd", 8.9999f64),
		("G7oV7X7FzJJ2g8TetMvAaC4GvM1ZCzPCnfSk5KcnVy2PJEF", 9f64),
		("GAQ75C8zHzVjWJLDH5PqFbLxfyck89wLJkhCzMZTXBFV5L7", 8.89f64),
		("GD8GVB4Ai4uCSzMCiX6C4vuY4DVkBctQRT6QMqD6y5mJfr9", 175f64),
		("GFgA4KV6Mm2TsHwEYtXfXZCf6QzebuN7c69UJGcRF2vCDTD", 9.3266f64),
		("GHTohCLUcLyB8w4xMr4TyDkvUzDnim6fadWfJ7edPke971d", 8.6666f64),
		("GJ2wraBjTtXH3m2TUxsVEc1bQdiuo6yiZUfczsws19uioX9", 8.8799f64),
		("GL2tM9Q6KL6XxMEWBg5jdjLezP4EzYsh3bqR1pjTmwDY1Mf", 83.499f64),
		("GLZdTryDdiHe3gVwFdncm2a6pKN6EiMxTGJQFHq5RxcNJZt", 93f64),
		("GLnjFi1U2jk6hDs7HCL1Y8XHsVLPHd2UAQh3AqRgpcUAUcn", 11.9933f64),
		("GRLkXUQUNqQMxKnNH1cM5TvWmu2H3yxGx9ohvJXSxRHUyJq", 15f64),
		("GRvRXw8jB2H9EGojmmjM4vZCeofuu91W2Zpbtr2QC5f7iLb", 11.48f64),
		("GUtkCnQKsVXzjqJvSZkiHtvdwZoEyukZrDE1iLTyV8F7tH6", 94.999f64),
		("GWrL9KsayVkniG2GmkYoTRDaFHhpTTSf6fL25e623bHVkRt", 17f64),
		("GbiPhuH4m4BCYiiLpqo2S2TyFxvgpfdSKWMdp9ZrbKdtXeM", 13.5f64),
		("GczUwD9zRdzJnTCr9kyREURmVeYpZAg1wNWrZMjqqimHcds", 8.5546f64),
		("GkS2m7UK6RSUkFiCHYPRtp8GCKdPtrBwP1wj7448gWoiut8", 0.0966f64),
		("Gnh63rW5fy3FAbnZQPkajyrQXxQGkg56Waoe2Ede1RjRjrJ", 0f64),
		("GnkMTVovNUith3JMaN5yvpuuwPzVeFbjM15wzg6LrAQSN6P", 0f64),
		("GnuSjJqE7VWMUGYsEEqsMioXwzNXfhu7FPMvdfKCiVazJwD", 21.3333f64),
		("GoUZE6g169BskxYVFV6prDUN4siKxa6Mr8VZD8B6TU9MEtX", 7.5f64),
		("Gy4LfGahVbR4eM8Sj2aCC4hHFk4fbUWGZCWQV5pjSGwJRZb", 21f64),
		("GyynnvzZzcJt24FQ6DNNg77Mgbfe8ddLdUiQG6nssoBVpmE", 9.7155f64),
		("H1eyZGi1DKxTGdtM8UQRA1RGEe6kGhvKC4onRpy97qKqiPE", 6.4933f64),
		("H2ZXJi5QAVB29d48oWgM1oBAmg2ngiofft4yEeyq58pViwt", 14.5f64),
		("H4YEv9v4DzU6WJsF2pxSHGcVxZEZDS761XWYsnRKX7P3aMQ", 14.8333f64),
		("H9eSvWe34vQDJAWckeTHWSqSChRat8bgKHG39GC1fjvEm7y", 0f64),
		("HJHs8fpzT916HTQgdGf6s69qp5zwNmZpTTnGXmm6qNqaQyW", 0f64),
		("HL8bEp8YicBdrUmJocCAWVLKUaR2dd1y6jnD934pbre3un1", 100f64),
		("HNnVpuhqgXRRDL8h4H39EgvvXtVYEtFSPi1XBWHcEW4cEYc", 110f64),
		("HTbdbMAMsFkUVeoMQ6pbH341DgQ1a4MpdCcaBu5N6uShJBq", 19f64),
		("HZWcWK1Px51bWgu1BD5d9M6A1djczFsdRWVwhVpRrU6VUJH", 34.6666f64),
		("Hbgxh54N33ApEZrpnYGhhvPJNYWD5LNprZodPVnSHq7QDDh", 110f64),
		("Hgh6jDVF9SXHs5pw6TfAXMWj4qHXpU6Q4dXh7h8zpx26cdw", 41f64),
		("HhH7NMA7FEkj327ZktSzkthknuQU591tLr1vX8zuc4E4JkB", 1.9933f64),
		("Hjjn1CmyGWKCswCmVZJFTFMJkruTtgeycEmvR4rWJYU7izQ", 18.6466f64),
		("HkRdC1w5XDvQQadAS2nL58mPRBCLyUCZAAiaV7DUWJgj7P8", 0f64),
		("Hn4y5xom4rBD49e2eMTpsEbqAJgE7uSR62VGLVv78ZqCuxT", 9.2161f64),
		("Hpaecetm5cBAJYDVqN9VeFUgVNQh3AAJAqUKAFW8PZGRst6", 9f64),
		("HsZTGLU5foma6bWoC7BGAs51aRu9TNc2RKRQBcfaHhJES1z", 72.3333f64),
		("HyXNSykBLZFMK8eAdys5L97ncetUhnStnLKMpuSp7kM9bCU", 14.5666f64),
		("J3hg1qmm6VeU6WWcxGQnKaMnD8wYAxnFQwaysE7RhVXbNvs", 7.4933f64),
		("J7XbTDFU4SRUaW6t6mYiK4tzDXtguhwa5ZK13EVCE5F2qak", 56.6666f64),
		("J9c2fcmRhhNaJAxA8yLMkxap7PEWuYc1UaaTqxunfKscjG3", 0f64),
		("JBfNxpntp7DaRM7pzg4XEm9TYX3gCaAVZvwgZW25ZzwXjy2", 0f64),
		("JEzQAbxmotcDNAXFFgcDcjgEF9gbuxaxer4bhGdKEfQqyRw", 0f64),
		("JF1SD3o1qZFdLBWmKTRHcHgBsXZRGGJhkwLeStYNoHs7ep1", 1.8933f64),
		("JFGuwWzqiyJZ3MPM8NRy7kK75MdxwSkh7GNZcsVd6BpZq2h", 20.3333f64),
		("D5WYdgC7f4W6jGCkQaQ3Lfe5P5F7JvfhYBAX9G6CBToMYe4", 0f64),
		("EJgdRddcYSd6XWnwr8oZkkzxJJX8SLwig38d5yxZSRgJGQZ", 0f64),
		("FXRK8xzufVJ45bCXuCCRr6FuS1dCfx9VjiuwqZDSqzEWgLm", 0f64),
		("Gt8ferkwFEX9jLbxpuhLYqKdNzP9KtjZcGYVN2mMbuCwhAP", 0f64),
		("HHCd33jjDBJJDZic7rbQZMuMaoe5ddYaRkaLf7tDURHpE56", 0f64),
		("E7b4mfFEhpnbw7iKdhvnBw7fXoQ1QFQE4TtTRhseWMU1H2E", 14.9000f64),
		("EGUE7VyrVLVSoaG43XVrEf3eKcgzJC8p3mFRtPUTMNLhdYs", 15.9000f64),
		("EtommijqrHDFWFvBxP515oUbkXx9vK2qohzrmwpCXbU7Yx2", 0f64),
		("GxzBsXcxZXXaKcLXj4CWGDdoCaXRVmcRkyiUAHFc1AfTMEV", 0f64),
		("H1heyw8DdkexFHAJ85GhuJr5om72Kx2DdK6sHsmb2f9ebUJ", 0f64),
		("J1Mf8RWcRWpuCRBAQVq6yRgb9exuHdKJdfV2NymtbXdkTF4", 13.9866f64),
		("FX6PCBQr4gtejvdR5dCdstrxmS3oFrzjZHtAKXSBumEse96", 0f64),
		("EdE26hU1nVmmVoteHWGKX8BuayykEg84iz7x3MCmzfocn2u", 0f64),
		("DzN3bMAAKKam6DBr9co4r1TBSj5X6Looh3JvNmfegiVrUgy", 0f64),
		("Dx2p54FSAxrqvFVN4ptvjV9LoUHkKQirM8SbLjD9PVnAonv", 0f64),
		("G6wtWujSHgT24UxXhDHVaVbnY5fBkUJLsjykyWBvtNs2aGQ", 0.0266666664f64),
		("DMrWWv31QiiDDESjGjyVcGhoCYMkPvrSTHHUDK5mR1riV2A", 0f64),
		("EkahQVDKRCe97ZT9TgS1YyfTzLgPah5PHa8NsfGFLsPKpjo", 0f64),
		("GhQ3gB8oaLZfjSd6gyeYZgymnJ2LEmvaUrSELBbva9342Y6", 0f64),
	];
	for (who, amount) in payouts.into_iter().filter(|&(_, amount)| amount > 0f64) {
		println!(
			"(hex![\"{}\"], {}u128),",
			HexDisplay::from(AsRef::<[u8; 32]>::as_ref(&AccountId32::from_string(who).unwrap())),
			(amount * 1_000_000_000_000f64).round() as u128,
		)
	}
}
