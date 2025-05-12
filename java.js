cron.schedule('*/ * * * * *', async () => {
    try {
       const metadata = await api.rpc.state.getMetadata();
       await api.registry.setMetadata(metadata);
       const blockHeader = await api.rpc.chain.getHeader();
       const finalized = await api.rpc.chain.getFinalizedHead();
       const isFinalized = finalized.eq(blockHeader.hash);
       const blockNumber = await blockHeader.number.toNumber();
       const lastBlock = await BlockHelper.getLastSavedBlockdb(
          Block,
          blockNumber
       );
       Logger.info('Last saved block ' + lastBlock);
       console.log(`Writing ${blockNumber - lastBlock} blocks to db`);

       for (let index = lastBlock; index <= blockNumber; index++) {
          let isNewBlock = true;
          const blockHash = await api.rpc.chain.getBlockHash(253751);
          // const blockPresent = await checkBlock(Block, 96138);
          // if (blockPresent) {
          //    continue
          // }
          const header = await api?.derive?.chain?.getHeader(blockHash);
          const block = await api.rpc.chain.getBlock(blockHash);
          let transactionCount = 0;
          let weight = 0;
          let deposit = 0;
          let events = 0;
          let transfer = 0;
          const transactions: any = {};
          const redisTransaction: any = [];
          const payout: any = [];
          const arr: any[] = [];
          const extrinsicCount = block.block.extrinsics.length;
          const save_Trans: any[] = [];
          block.block.header.forEach((ex: any) => {
             arr.push(ex.toHuman());
          });
          const block_number = JSON.parse(block?.block?.header?.number);
          const timestamp = await api.query.timestamp.now.at(
             blockHash.toHex()
          );
          const timestampMs = timestamp.toNumber();
          const timestampDate = new Date(timestampMs);
          const transactionTimeStamp = +timestampDate;
          const allRecords = await api?.query?.system?.events?.at(
             block?.block?.header?.hash
          );
          const createTimeStamp = Number(
             block?.block?.extrinsics[0].method.args[0]
          );
          let totalTransactionSize = 0;
          block?.block?.extrinsics?.forEach(
             (
                { method: { method, section } }: any,
                index: any,
                extrinsic: any
             ) => {
                let eraIndex = '';
                let isEvm = false;
                totalTransactionSize = extrinsic.encodedLength;
                allRecords
                   .filter(
                      ({ phase }: any) =>
                         phase.isApplyExtrinsic &&
                         phase.asApplyExtrinsic.eq(index)
                   )
                   .map(async (val: any) => {
                      const { event } = val;
                      const lowerSection = event?.section?.toLowerCase();
                      const lowerMethod = event?.method?.toLowerCase();
                      events = events + 1;
                      if (
                         (lowerSection == 'multisig' &&
                            lowerMethod == 'multisigaddress') ||
                         lowerMethod == 'multisigcancelled'
                      ) {
                         multisigAddress = event?.toHuman().data?.multisig;
                      }
                      if (event.toHuman().method == 'Seconded') {
                         ProposalHelper.getDemocracyProposals(await api);
                      }
                      if (event.toHuman().method == 'Voted') {
                         if (event.toHuman().section == 'council') {
                            ProposalHelper.saveTreasuryVote(
                               await api,
                               event,
                               createTimeStamp
                            );
                         } else if (
                            event.toHuman().section == 'technicalCommittee'
                         ) {
                            ProposalHelper.getTechnicalProposals(
                               await api
                            );
                         } else {
                            await ReferndaHelper.saveVote(
                               api,
                               event,
                               createTimeStamp
                            );
                         }
                      }
                      if (
                         ((section == 'evm' || section == 'ethereum') &&
                            !isEvm &&
                            method == 'transact') ||
                         event.toHuman().method == 'Log'
                      ) {
                         console.log('INDEX PAGE CREATOR DATA  ==============>>>>>>',event.toHuman());
                         
                         isEvm = true;
                         const T_countEvm =
                            await TransactionHelper.getLastSavedTransacdb();
                         transactionCount += await evmEvent(
                            section,
                            method,
                            block_number,
                            T_countEvm,
                            allRecords,
                            index
                         );
                         if (
                            event.toHuman().section == 'evm' &&
                            event.toHuman().method == 'Log'
                         ) {
                            try {
                               await TokenHelper.createToken(event);
                            } catch (error) {
                               console.error(
                                  'Error creating token:',
                                  error
                               );
                            }
                         }
                      }
                      if (
                         event?.method?.toLowerCase() == 'extrinsicfailed'
                      ) {
                         const [dispatchError] = event.data;
                         let errorInfo;
                         if (dispatchError.isModule) {
                            const decoded = api.registry.findMetaError(
                               dispatchError.asModule
                            );
                            errorInfo = `${decoded.section}.${decoded.name}`;
                         } else {
                            errorInfo = dispatchError.toString();
                         }
                         let temptransaction = '';
                         const data = JSON?.parse(
                            block?.block?.extrinsics[index]
                         );

                         const from = nullValidation(
                            data?.signature?.signer
                         );
                         const to = nullValidation(
                            data?.method?.args?.dest
                         );
                         const value = extractValue(data);

                         const txFee = transactions[
                            `${block.block.extrinsics[index].hash}`
                         ]?.txFee
                            ? transactions[
                                 `${block.block.extrinsics[index].hash}`
                              ].txFee
                            : 0;
                         transactionCount = transactionCount + 1;
                         temptransaction = `${temptransaction}(UUID_TO_BIN(UUID()),'${from}','${to}','${value}','${
                            block.block.extrinsics[index].hash
                         }','${block_number}',now(),now(),'${errorInfo}','${section}.${method}','failed','${txFee}','${
                            data?.method?.args?.memo
                               ? data?.method?.args?.memo
                               : null
                         }
                        ','null','null'`;
                         transactions[
                            `${block.block.extrinsics[index].hash}`
                         ] = temptransaction;
                         save_Trans.push({
                            from: nullValidation(from),
                            to: nullValidation(to),
                            value: value,
                            txnHash: nullValidation(
                               block.block.extrinsics[index].hash
                            ),
                            block_number: block_number,
                            reason: errorInfo,
                            sectionmethod: `${section}.${method}`,
                            status: 'failed',
                            txfee: nullValidation(txFee),
                            count: 0,
                            createdAt: new Date(),
                            updatedAt: new Date(),
                            timestamp: timestamp?.toString(),
                         });
                      } else if (
                         lowerSection == 'multisig' &&
                         lowerMethod == 'multisigcancelled'
                      ) {
                         isMultisigCancelleAddress =
                            event?.data.toHuman().multisig;
                      }
                    }}}}