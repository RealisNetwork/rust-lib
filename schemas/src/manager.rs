use crate::common::*;
use crate::generated_schemas::*;
use crate::Schema;
use error_registry::{
    generated_errors::{Common, Validation},
    BaseError,
};
use jsonschema::JSONSchema;
use serde_json::{json, Value};
pub struct SchemaManager;
impl SchemaManager {
    pub fn get_params_schema(agent: &str, method: &str) -> Option<Value> {
        match (agent, method) {
            ("achievements", "achievement_achievementComplete") => {
                Some(AchievementsAchievementAchievementCompleteParams::schema())
            }
            ("achievements", "achievement_getAllAchievements") => {
                Some(AchievementsAchievementGetAllAchievementsParams::schema())
            }
            ("achievements", "achievement_getUsersAchievements") => {
                Some(AchievementsAchievementGetUsersAchievementsParams::schema())
            }
            ("admin", "action_undo") => Some(AdminActionUndoParams::schema()),
            ("admin", "action_getActionList") => Some(AdminActionGetActionListParams::schema()),
            ("admin", "action_deleteByActionId") => {
                Some(AdminActionDeleteByActionIdParams::schema())
            }
            ("admin", "action_getAllByFilterList") => {
                Some(AdminActionGetAllByFilterListParams::schema())
            }
            ("admin", "confirmation_addSolutionAdmin") => {
                Some(AdminConfirmationAddSolutionAdminParams::schema())
            }
            ("admin", "confirmation_deleteAction") => {
                Some(AdminConfirmationDeleteActionParams::schema())
            }
            ("admin", "confirmation_getAllActions") => {
                Some(AdminConfirmationGetAllActionsParams::schema())
            }
            ("admin", "confirmation_getNotConfirmedActions") => {
                Some(AdminConfirmationGetNotConfirmedActionsParams::schema())
            }
            ("admin", "confirmation_add") => Some(AdminConfirmationAddParams::schema()),
            ("admin", "confirmation_addSolutionAdminForMany") => {
                Some(AdminConfirmationAddSolutionAdminForManyParams::schema())
            }
            ("admin", "mailTemplate_create") => Some(AdminMailTemplateCreateParams::schema()),
            ("admin", "mailTemplate_delete") => Some(AdminMailTemplateDeleteParams::schema()),
            ("admin", "mailTemplate_getByKey") => Some(AdminMailTemplateGetByKeyParams::schema()),
            ("admin", "mailTemplate_change") => Some(AdminMailTemplateChangeParams::schema()),
            ("admin", "mailTemplate_getAll") => Some(AdminMailTemplateGetAllParams::schema()),
            ("admin", "option_set") => Some(AdminOptionSetParams::schema()),
            ("admin", "option_unset") => Some(AdminOptionUnsetParams::schema()),
            ("admin", "option_get") => Some(AdminOptionGetParams::schema()),
            ("admin", "option_getList") => Some(AdminOptionGetListParams::schema()),
            ("admin", "option_getByScope") => Some(AdminOptionGetByScopeParams::schema()),
            ("admin", "option_getAllByFilter") => Some(AdminOptionGetAllByFilterParams::schema()),
            ("admin", "option_update") => Some(AdminOptionUpdateParams::schema()),
            ("admin", "option_updateAll") => Some(AdminOptionUpdateAllParams::schema()),
            ("admin", "permission_add") => Some(AdminPermissionAddParams::schema()),
            ("admin", "permission_delete") => Some(AdminPermissionDeleteParams::schema()),
            ("admin", "permission_updatePermissions") => {
                Some(AdminPermissionUpdatePermissionsParams::schema())
            }
            ("admin", "role_add") => Some(AdminRoleAddParams::schema()),
            ("admin", "role_delete") => Some(AdminRoleDeleteParams::schema()),
            ("admin", "role_getUsersByRole") => Some(AdminRoleGetUsersByRoleParams::schema()),
            ("admin", "role_getPermissionsByRoleName") => {
                Some(AdminRoleGetPermissionsByRoleNameParams::schema())
            }
            ("admin", "role_getAllRoles") => Some(AdminRoleGetAllRolesParams::schema()),
            ("admin", "role_getRoleByName") => Some(AdminRoleGetRoleByNameParams::schema()),
            ("admin", "userRole_add") => Some(AdminUserRoleAddParams::schema()),
            ("admin", "userRole_getAll") => Some(AdminUserRoleGetAllParams::schema()),
            ("admin", "userRole_deleteUserRole") => {
                Some(AdminUserRoleDeleteUserRoleParams::schema())
            }
            ("admin", "userRole_getAllWithNickname") => {
                Some(AdminUserRoleGetAllWithNicknameParams::schema())
            }
            ("admin", "userRole_getMyRole") => Some(AdminUserRoleGetMyRoleParams::schema()),
            ("admin", "userRole_getAllUsersWithNickname") => {
                Some(AdminUserRoleGetAllUsersWithNicknameParams::schema())
            }
            ("admin", "userRole_isAdmin") => Some(AdminUserRoleIsAdminParams::schema()),
            ("analytics", "analytics_send") => Some(AnalyticsAnalyticsSendParams::schema()),
            ("auth", "admin_login") => Some(AuthAdminLoginParams::schema()),
            ("auth", "admin_addUserRole") => Some(AuthAdminAddUserRoleParams::schema()),
            ("auth", "admin_deleteUserRole") => Some(AuthAdminDeleteUserRoleParams::schema()),
            ("auth", "admin_updateUserRole") => Some(AuthAdminUpdateUserRoleParams::schema()),
            ("auth", "admin_getUserRole") => Some(AuthAdminGetUserRoleParams::schema()),
            ("auth", "admin_getMyRole") => Some(AuthAdminGetMyRoleParams::schema()),
            ("auth", "admin_getUsers") => Some(AuthAdminGetUsersParams::schema()),
            ("auth", "auth_login") => Some(AuthAuthLoginParams::schema()),
            ("auth", "auth_socialNetworkAuth") => Some(AuthAuthSocialNetworkAuthParams::schema()),
            ("auth", "auth_changePassword") => Some(AuthAuthChangePasswordParams::schema()),
            ("auth", "auth_sendRequestToResetPassword") => {
                Some(AuthAuthSendRequestToResetPasswordParams::schema())
            }
            ("auth", "auth_resetPassword") => Some(AuthAuthResetPasswordParams::schema()),
            ("auth", "auth_sendRequestToDeleteAccount") => {
                Some(AuthAuthSendRequestToDeleteAccountParams::schema())
            }
            ("auth", "auth_removeAccount") => Some(AuthAuthRemoveAccountParams::schema()),
            ("auth", "auth_getAccessTokenByRefresh") => {
                Some(AuthAuthGetAccessTokenByRefreshParams::schema())
            }
            ("auth", "auth_getAllMyActiveSessions") => {
                Some(AuthAuthGetAllMyActiveSessionsParams::schema())
            }
            ("auth", "auth_confirmPassword") => Some(AuthAuthConfirmPasswordParams::schema()),
            ("auth", "auth_deleteMyUser") => Some(AuthAuthDeleteMyUserParams::schema()),
            ("auth", "auth_getUserInfoByToken") => Some(AuthAuthGetUserInfoByTokenParams::schema()),
            ("auth", "auth_createRequestToConfirmEmail") => {
                Some(AuthAuthCreateRequestToConfirmEmailParams::schema())
            }
            ("auth", "auth_confirmEmailByCode") => Some(AuthAuthConfirmEmailByCodeParams::schema()),
            ("auth", "auth_confirmEmailByHash") => Some(AuthAuthConfirmEmailByHashParams::schema()),
            ("auth", "auth_deleteEmailRequest") => Some(AuthAuthDeleteEmailRequestParams::schema()),
            ("auth", "auth_setPassword") => Some(AuthAuthSetPasswordParams::schema()),
            ("auth", "auth_disableUser") => Some(AuthAuthDisableUserParams::schema()),
            ("auth", "auth_resentConfirmationMail") => {
                Some(AuthAuthResentConfirmationMailParams::schema())
            }
            ("auth", "auth_createRequestToChangeEmail") => {
                Some(AuthAuthCreateRequestToChangeEmailParams::schema())
            }
            ("auth", "auth_changeMyEmail") => Some(AuthAuthChangeMyEmailParams::schema()),
            ("auth", "auth_loginWithFacebook") => Some(AuthAuthLoginWithFacebookParams::schema()),
            ("auth", "auth_logout") => Some(AuthAuthLogoutParams::schema()),
            ("auth", "auth_emptyMethod") => Some(AuthAuthEmptyMethodParams::schema()),
            ("auth", "auth_assignProviderAccountToDeviceId") => {
                Some(AuthAuthAssignProviderAccountToDeviceIdParams::schema())
            }
            ("auth", "auth_resetUserProgress") => Some(AuthAuthResetUserProgressParams::schema()),
            ("auth", "authDevice_getOrCreateInternalUserId") => {
                Some(AuthAuthDeviceGetOrCreateInternalUserIdParams::schema())
            }
            ("auth", "authDevice_isNewClient") => Some(AuthAuthDeviceIsNewClientParams::schema()),
            ("auth", "authDevice_isAuthorized") => Some(AuthAuthDeviceIsAuthorizedParams::schema()),
            ("auth", "authDevice_getClientInfo") => {
                Some(AuthAuthDeviceGetClientInfoParams::schema())
            }
            ("auth", "authDevice_updateInternalIdByDeviceId") => {
                Some(AuthAuthDeviceUpdateInternalIdByDeviceIdParams::schema())
            }
            ("auth", "authDevice_getClientStatusByInternalUserId") => {
                Some(AuthAuthDeviceGetClientStatusByInternalUserIdParams::schema())
            }
            ("auth", "ban_ban") => Some(AuthBanBanParams::schema()),
            ("auth", "ban_unBan") => Some(AuthBanUnBanParams::schema()),
            ("auth", "ban_getUserBan") => Some(AuthBanGetUserBanParams::schema()),
            ("auth", "instantMigration_initMigration") => {
                Some(AuthInstantMigrationInitMigrationParams::schema())
            }
            ("auth", "instantMigration_stopMigration") => {
                Some(AuthInstantMigrationStopMigrationParams::schema())
            }
            ("auth", "role_add") => Some(AuthRoleAddParams::schema()),
            ("auth", "role_delete") => Some(AuthRoleDeleteParams::schema()),
            ("auth", "role_update") => Some(AuthRoleUpdateParams::schema()),
            ("auth", "role_getAllRoles") => Some(AuthRoleGetAllRolesParams::schema()),
            ("auth", "role_getRoleByName") => Some(AuthRoleGetRoleByNameParams::schema()),
            ("auth", "twoFactor_getSecret") => Some(AuthTwoFactorGetSecretParams::schema()),
            ("auth", "twoFactor_verifyCode") => Some(AuthTwoFactorVerifyCodeParams::schema()),
            ("auth", "twoFactor_disableTwoFA") => Some(AuthTwoFactorDisableTwoFaParams::schema()),
            ("auth", "twoFactor_enableTwoFA") => Some(AuthTwoFactorEnableTwoFaParams::schema()),
            ("auth", "twoFactor_isRequiredTwoFA") => {
                Some(AuthTwoFactorIsRequiredTwoFaParams::schema())
            }
            ("auth", "twoFactor_getStatus") => Some(AuthTwoFactorGetStatusParams::schema()),
            ("auth", "twoFactor_sendRequestToDeleteTwoFA") => {
                Some(AuthTwoFactorSendRequestToDeleteTwoFaParams::schema())
            }
            ("auth", "twoFactor_deleteTwoFA") => Some(AuthTwoFactorDeleteTwoFaParams::schema()),
            ("balances", "balances_freeExperienceAddedNotification") => {
                Some(BalancesBalancesFreeExperienceAddedNotificationParams::schema())
            }
            ("balances", "balances_getSoftCurrencyBalance") => {
                Some(BalancesBalancesGetSoftCurrencyBalanceParams::schema())
            }
            ("balances", "balances_increaseSoftCurrency") => {
                Some(BalancesBalancesIncreaseSoftCurrencyParams::schema())
            }
            ("balances", "balances_decreaseSoftCurrency") => {
                Some(BalancesBalancesDecreaseSoftCurrencyParams::schema())
            }
            ("balances", "balances_addFreeExperience") => {
                Some(BalancesBalancesAddFreeExperienceParams::schema())
            }
            ("balances", "balances_useFreeExperience") => {
                Some(BalancesBalancesUseFreeExperienceParams::schema())
            }
            ("balances", "balances_getBalancesByUserId") => {
                Some(BalancesBalancesGetBalancesByUserIdParams::schema())
            }
            ("balances", "balances_getFreeExperienceBalance") => {
                Some(BalancesBalancesGetFreeExperienceBalanceParams::schema())
            }
            ("battle-pass", "battlePassEndpoints_receiveReward") => {
                Some(BattlePassBattlePassEndpointsReceiveRewardParams::schema())
            }
            ("battle-pass", "battlePassEndpoints_getBattlePassData") => {
                Some(BattlePassBattlePassEndpointsGetBattlePassDataParams::schema())
            }
            ("battle-pass", "battlePassEndpoints_getCurrentBattlePassExperience") => {
                Some(BattlePassBattlePassEndpointsGetCurrentBattlePassExperienceParams::schema())
            }
            ("battle-pass", "battlePassEndpoints_getBattlePassSeasonInfo") => {
                Some(BattlePassBattlePassEndpointsGetBattlePassSeasonInfoParams::schema())
            }
            ("battle-pass", "battlePassEndpoints_getBattlePassPrices") => {
                Some(BattlePassBattlePassEndpointsGetBattlePassPricesParams::schema())
            }
            ("battle-pass", "battlePassEndpoints_receiveOldBattlePassRewards") => {
                Some(BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsParams::schema())
            }
            ("battle-pass", "battlePassEndpoints_purchaseBattlePassExperienceToLevelup") => Some(
                BattlePassBattlePassEndpointsPurchaseBattlePassExperienceToLevelupParams::schema(),
            ),
            ("battle-pass", "battlePassEndpoints_purchaseBattlePassPremium") => {
                Some(BattlePassBattlePassEndpointsPurchaseBattlePassPremiumParams::schema())
            }
            ("binance-wallet", "wallet_createWallet") => {
                Some(BinanceWalletWalletCreateWalletParams::schema())
            }
            ("binance-wallet", "wallet_getAddressByUserId") => {
                Some(BinanceWalletWalletGetAddressByUserIdParams::schema())
            }
            ("binance-wallet", "wallet_getUserIdByAddress") => {
                Some(BinanceWalletWalletGetUserIdByAddressParams::schema())
            }
            ("bingo", "bingo_getBingoData") => Some(BingoBingoGetBingoDataParams::schema()),
            ("bingo", "bingo_craftBingoRecipe") => Some(BingoBingoCraftBingoRecipeParams::schema()),
            ("bingo", "bingo_getBingoJackpotPool") => {
                Some(BingoBingoGetBingoJackpotPoolParams::schema())
            }
            ("bingo", "bingo_getBingoJackpotWinnersInfo") => {
                Some(BingoBingoGetBingoJackpotWinnersInfoParams::schema())
            }
            ("bingo", "bingo_getCurrentBingoSeason") => {
                Some(BingoBingoGetCurrentBingoSeasonParams::schema())
            }
            ("blog", "blog_create") => Some(BlogBlogCreateParams::schema()),
            ("blog", "blog_update") => Some(BlogBlogUpdateParams::schema()),
            ("blog", "blog_delete") => Some(BlogBlogDeleteParams::schema()),
            ("blog", "blog_getAll") => Some(BlogBlogGetAllParams::schema()),
            ("blog", "blog_get") => Some(BlogBlogGetParams::schema()),
            ("blog", "blog_getByUrl") => Some(BlogBlogGetByUrlParams::schema()),
            ("blog", "blog_getPinned") => Some(BlogBlogGetPinnedParams::schema()),
            ("blog", "blog_getAllByFilter") => Some(BlogBlogGetAllByFilterParams::schema()),
            ("blog", "blog_getAllWithCategoryList") => {
                Some(BlogBlogGetAllWithCategoryListParams::schema())
            }
            ("blog", "category_create") => Some(BlogCategoryCreateParams::schema()),
            ("blog", "category_update") => Some(BlogCategoryUpdateParams::schema()),
            ("blog", "category_delete") => Some(BlogCategoryDeleteParams::schema()),
            ("blog", "category_getAll") => Some(BlogCategoryGetAllParams::schema()),
            ("blog", "category_get") => Some(BlogCategoryGetParams::schema()),
            ("blog", "poll_add") => Some(BlogPollAddParams::schema()),
            ("blog", "poll_get") => Some(BlogPollGetParams::schema()),
            ("blog", "poll_getAll") => Some(BlogPollGetAllParams::schema()),
            ("blog", "poll_update") => Some(BlogPollUpdateParams::schema()),
            ("blog", "poll_delete") => Some(BlogPollDeleteParams::schema()),
            ("blog", "vote_addVote") => Some(BlogVoteAddVoteParams::schema()),
            ("blog", "vote_getAllVotesByPollId") => {
                Some(BlogVoteGetAllVotesByPollIdParams::schema())
            }
            ("blog", "vote_isVoted") => Some(BlogVoteIsVotedParams::schema()),
            ("cats", "lobby_achievementComplete") => {
                Some(CatsLobbyAchievementCompleteParams::schema())
            }
            ("cats", "lobby_getAllAchievements") => {
                Some(CatsLobbyGetAllAchievementsParams::schema())
            }
            ("cats", "lobby_getUsersAchievements") => {
                Some(CatsLobbyGetUsersAchievementsParams::schema())
            }
            ("cats", "lobby_adventureMapLevelUp") => {
                Some(CatsLobbyAdventureMapLevelUpParams::schema())
            }
            ("cats", "lobby_getAdventureMapStats") => {
                Some(CatsLobbyGetAdventureMapStatsParams::schema())
            }
            ("cats", "lobby_getApplicationSettings") => {
                Some(CatsLobbyGetApplicationSettingsParams::schema())
            }
            ("cats", "lobby_craftBingoRecipe") => Some(CatsLobbyCraftBingoRecipeParams::schema()),
            ("cats", "lobby_getBingoData") => Some(CatsLobbyGetBingoDataParams::schema()),
            ("cats", "lobby_getUsersCards") => Some(CatsLobbyGetUsersCardsParams::schema()),
            ("cats", "lobby_upgradeCard") => Some(CatsLobbyUpgradeCardParams::schema()),
            ("cats", "lobby_getClientVersion") => Some(CatsLobbyGetClientVersionParams::schema()),
            ("cats", "lobby_getConfigById") => Some(CatsLobbyGetConfigByIdParams::schema()),
            ("cats", "lobby_updateConfig") => Some(CatsLobbyUpdateConfigParams::schema()),
            ("cats", "lobby_buyCat") => Some(CatsLobbyBuyCatParams::schema()),
            ("cats", "lobby_buyScience") => Some(CatsLobbyBuyScienceParams::schema()),
            ("cats", "lobby_buySkill") => Some(CatsLobbyBuySkillParams::schema()),
            ("cats", "lobby_buyUsualLootbox") => Some(CatsLobbyBuyUsualLootboxParams::schema()),
            ("cats", "lobby_getBingoJackpotPool") => {
                Some(CatsLobbyGetBingoJackpotPoolParams::schema())
            }
            ("cats", "lobby_getBingoJackpotWinnersInfo") => {
                Some(CatsLobbyGetBingoJackpotWinnersInfoParams::schema())
            }
            ("cats", "lobby_getLootboxJackpotWinnersInfo") => {
                Some(CatsLobbyGetLootboxJackpotWinnersInfoParams::schema())
            }
            ("cats", "lobby_getLootboxJackpotPool") => {
                Some(CatsLobbyGetLootboxJackpotPoolParams::schema())
            }
            ("cats", "lobby_getPreviousTournamentStats") => {
                Some(CatsLobbyGetPreviousTournamentStatsParams::schema())
            }
            ("cats", "lobby_getSoftLeaderboardStats") => {
                Some(CatsLobbyGetSoftLeaderboardStatsParams::schema())
            }
            ("cats", "lobby_getTournamentLeaderboardStats") => {
                Some(CatsLobbyGetTournamentLeaderboardStatsParams::schema())
            }
            ("cats", "lobby_getLootboxesInfo") => Some(CatsLobbyGetLootboxesInfoParams::schema()),
            ("cats", "lobby_getUsualLootboxTime") => {
                Some(CatsLobbyGetUsualLootboxTimeParams::schema())
            }
            ("cats", "lobby_openLootbox") => Some(CatsLobbyOpenLootboxParams::schema()),
            ("cats", "lobby_getAvailableMembershipAmount") => {
                Some(CatsLobbyGetAvailableMembershipAmountParams::schema())
            }
            ("cats", "lobby_getOffersAndBonuses") => {
                Some(CatsLobbyGetOffersAndBonusesParams::schema())
            }
            ("cats", "lobby_refuseOffer") => Some(CatsLobbyRefuseOfferParams::schema()),
            ("cats", "lobby_saveTutorialProgress") => {
                Some(CatsLobbySaveTutorialProgressParams::schema())
            }
            ("cats", "lobby_saveUsersProgress") => Some(CatsLobbySaveUsersProgressParams::schema()),
            ("cats", "lobby_getUsersReferrals") => Some(CatsLobbyGetUsersReferralsParams::schema()),
            ("cats", "lobby_setReferrer") => Some(CatsLobbySetReferrerParams::schema()),
            ("cats", "lobby_getServerTime") => Some(CatsLobbyGetServerTimeParams::schema()),
            ("cats", "lobby_keepAlive") => Some(CatsLobbyKeepAliveParams::schema()),
            ("cats", "lobby_getSpinRewards") => Some(CatsLobbyGetSpinRewardsParams::schema()),
            ("cats", "lobby_useSpin") => Some(CatsLobbyUseSpinParams::schema()),
            ("cats", "lobby_completeTask") => Some(CatsLobbyCompleteTaskParams::schema()),
            ("cats", "lobby_getCurrentTasks") => Some(CatsLobbyGetCurrentTasksParams::schema()),
            ("cats", "lobby_getUsersCompletedTasks") => {
                Some(CatsLobbyGetUsersCompletedTasksParams::schema())
            }
            ("cats", "lobby_userValidation") => Some(CatsLobbyUserValidationParams::schema()),
            ("cats", "lobby_getBadTransactions") => {
                Some(CatsLobbyGetBadTransactionsParams::schema())
            }
            ("cats", "lobby_getUserIdByTransactionId") => {
                Some(CatsLobbyGetUserIdByTransactionIdParams::schema())
            }
            ("cats", "lobby_getUsersInappPurchases") => {
                Some(CatsLobbyGetUsersInappPurchasesParams::schema())
            }
            ("cats", "lobby_resetDeviceId") => Some(CatsLobbyResetDeviceIdParams::schema()),
            ("cats", "lobby_unlinkAccount") => Some(CatsLobbyUnlinkAccountParams::schema()),
            ("cats", "lobby_getProfile") => Some(CatsLobbyGetProfileParams::schema()),
            ("cats", "lobby_getSoulPrice") => Some(CatsLobbyGetSoulPriceParams::schema()),
            ("cats", "lobby_purchaseValidation") => {
                Some(CatsLobbyPurchaseValidationParams::schema())
            }
            ("cats", "lobby_retrieveUsersProgress") => {
                Some(CatsLobbyRetrieveUsersProgressParams::schema())
            }
            ("cats", "lobby_shareScore") => Some(CatsLobbyShareScoreParams::schema()),
            ("cats", "lobby_setUserAppMetricaDeviceId") => {
                Some(CatsLobbySetUserAppMetricaDeviceIdParams::schema())
            }
            ("cats", "lobby_getJackpotWinnersInfo") => {
                Some(CatsLobbyGetJackpotWinnersInfoParams::schema())
            }
            ("cats", "lobby_getProfileInfo") => Some(CatsLobbyGetProfileInfoParams::schema()),
            ("cats", "lobby_applicationInitialization") => {
                Some(CatsLobbyApplicationInitializationParams::schema())
            }
            ("catsAndDragons", "wrapper_getNicknameChangePrice") => {
                Some(CatsAndDragonsWrapperGetNicknameChangePriceParams::schema())
            }
            ("catsAndDragons", "wrapper_nicknameChange") => {
                Some(CatsAndDragonsWrapperNicknameChangeParams::schema())
            }
            ("catsAndDragons", "wrapper_collectNicknames") => {
                Some(CatsAndDragonsWrapperCollectNicknamesParams::schema())
            }
            ("catsAndDragons", "wrapper_getUserIdByDeviceId") => {
                Some(CatsAndDragonsWrapperGetUserIdByDeviceIdParams::schema())
            }
            ("catsAndDragons", "wrapper_getReferrals") => {
                Some(CatsAndDragonsWrapperGetReferralsParams::schema())
            }
            ("catsAndDragons", "wrapper_getNickname") => {
                Some(CatsAndDragonsWrapperGetNicknameParams::schema())
            }
            ("catsAndDragons", "wrapper_getUserInfo") => {
                Some(CatsAndDragonsWrapperGetUserInfoParams::schema())
            }
            ("catsAndDragons", "wrapper_getMembershipsAndPricesAmount") => {
                Some(CatsAndDragonsWrapperGetMembershipsAndPricesAmountParams::schema())
            }
            ("catsAndDragons", "wrapper_getUserMembershipInfo") => {
                Some(CatsAndDragonsWrapperGetUserMembershipInfoParams::schema())
            }
            ("catsAndDragons", "wrapper_emptyMethod") => {
                Some(CatsAndDragonsWrapperEmptyMethodParams::schema())
            }
            ("cd-balances", "balance_getBalancesByUserId") => {
                Some(CdBalancesBalanceGetBalancesByUserIdParams::schema())
            }
            ("cd-balances", "balance_increaseBalanceByUserIdAndCurrency") => {
                Some(CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyParams::schema())
            }
            ("cd-balances", "balance_decreaseBalanceByUserIdAndCurrency") => {
                Some(CdBalancesBalanceDecreaseBalanceByUserIdAndCurrencyParams::schema())
            }
            ("cd-balances", "balance_getBalanceByUserIdAndCurrency") => {
                Some(CdBalancesBalanceGetBalanceByUserIdAndCurrencyParams::schema())
            }
            ("cd-config", "config_addConfig") => Some(CdConfigConfigAddConfigParams::schema()),
            ("cd-config", "config_getConfigByKey") => {
                Some(CdConfigConfigGetConfigByKeyParams::schema())
            }
            ("cd-user", "spinEndpoints_spin") => Some(CdUserSpinEndpointsSpinParams::schema()),
            ("cd-user", "spinEndpoints_getSpinInfo") => {
                Some(CdUserSpinEndpointsGetSpinInfoParams::schema())
            }
            ("cron", "cron_create") => Some(CronCronCreateParams::schema()),
            ("cron", "cron_update") => Some(CronCronUpdateParams::schema()),
            ("cron", "cron_get") => Some(CronCronGetParams::schema()),
            ("cron", "cron_delete") => Some(CronCronDeleteParams::schema()),
            ("dragocats-balancer", "balancer_enterQueue") => {
                Some(DragocatsBalancerBalancerEnterQueueParams::schema())
            }
            ("dragocats-balancer", "balancer_leaveQueue") => {
                Some(DragocatsBalancerBalancerLeaveQueueParams::schema())
            }
            ("dragocats-balancer", "balancer_serverStarted") => {
                Some(DragocatsBalancerBalancerServerStartedParams::schema())
            }
            ("dragocats-balancer", "balancer_serverStopped") => {
                Some(DragocatsBalancerBalancerServerStoppedParams::schema())
            }
            ("dragocats-lobby", "stats_getAttributesByUnitId") => {
                Some(DragocatsLobbyStatsGetAttributesByUnitIdParams::schema())
            }
            ("dragocats-lobby", "unitEndpoints_startBattle") => {
                Some(DragocatsLobbyUnitEndpointsStartBattleParams::schema())
            }
            ("dragocats-lobby", "unitEndpoints_equipUnit") => {
                Some(DragocatsLobbyUnitEndpointsEquipUnitParams::schema())
            }
            ("dragocats-lobby", "unitEndpoints_unEquipUnit") => {
                Some(DragocatsLobbyUnitEndpointsUnEquipUnitParams::schema())
            }
            ("dragocats-lobby", "unitEndpoints_getEquipedUnits") => {
                Some(DragocatsLobbyUnitEndpointsGetEquipedUnitsParams::schema())
            }
            ("dragocats-product-factory", "generator_generate") => {
                Some(DragocatsProductFactoryGeneratorGenerateParams::schema())
            }
            ("dragocats-product-factory", "lootbox_open") => {
                Some(DragocatsProductFactoryLootboxOpenParams::schema())
            }
            ("dragocats-product-factory", "products_getInfoByProductId") => {
                Some(DragocatsProductFactoryProductsGetInfoByProductIdParams::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_getUserInventory") => {
                Some(DragocatsStorageInventoryEndpointsGetUserInventoryParams::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_addLootboxNotification") => {
                Some(DragocatsStorageInventoryEndpointsAddLootboxNotificationParams::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_addUnitNotification") => {
                Some(DragocatsStorageInventoryEndpointsAddUnitNotificationParams::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_openLootbox") => {
                Some(DragocatsStorageInventoryEndpointsOpenLootboxParams::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_openLootboxNotification") => {
                Some(DragocatsStorageInventoryEndpointsOpenLootboxNotificationParams::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_equipUnit") => {
                Some(DragocatsStorageInventoryEndpointsEquipUnitParams::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_unEquipUnit") => {
                Some(DragocatsStorageInventoryEndpointsUnEquipUnitParams::schema())
            }
            ("dragocats-storage", "unitEndpoints_getByUnitId") => {
                Some(DragocatsStorageUnitEndpointsGetByUnitIdParams::schema())
            }
            ("dragons", "lobby_achievementComplete") => {
                Some(DragonsLobbyAchievementCompleteParams::schema())
            }
            ("dragons", "lobby_getAllAchievements") => {
                Some(DragonsLobbyGetAllAchievementsParams::schema())
            }
            ("dragons", "lobby_getUsersAchievements") => {
                Some(DragonsLobbyGetUsersAchievementsParams::schema())
            }
            ("dragons", "lobby_adventureMapLevelUp") => {
                Some(DragonsLobbyAdventureMapLevelUpParams::schema())
            }
            ("dragons", "lobby_getAdventureMapStats") => {
                Some(DragonsLobbyGetAdventureMapStatsParams::schema())
            }
            ("dragons", "lobby_getApplicationSettings") => {
                Some(DragonsLobbyGetApplicationSettingsParams::schema())
            }
            ("dragons", "lobby_craftBingoRecipe") => {
                Some(DragonsLobbyCraftBingoRecipeParams::schema())
            }
            ("dragons", "lobby_getBingoData") => Some(DragonsLobbyGetBingoDataParams::schema()),
            ("dragons", "lobby_getUsersCards") => Some(DragonsLobbyGetUsersCardsParams::schema()),
            ("dragons", "lobby_upgradeCard") => Some(DragonsLobbyUpgradeCardParams::schema()),
            ("dragons", "lobby_getClientVersion") => {
                Some(DragonsLobbyGetClientVersionParams::schema())
            }
            ("dragons", "lobby_getConfigById") => Some(DragonsLobbyGetConfigByIdParams::schema()),
            ("dragons", "lobby_updateConfig") => Some(DragonsLobbyUpdateConfigParams::schema()),
            ("dragons", "lobby_buyCat") => Some(DragonsLobbyBuyCatParams::schema()),
            ("dragons", "lobby_buyScience") => Some(DragonsLobbyBuyScienceParams::schema()),
            ("dragons", "lobby_buySkill") => Some(DragonsLobbyBuySkillParams::schema()),
            ("dragons", "lobby_buyUsualLootbox") => {
                Some(DragonsLobbyBuyUsualLootboxParams::schema())
            }
            ("dragons", "lobby_getBingoJackpotPool") => {
                Some(DragonsLobbyGetBingoJackpotPoolParams::schema())
            }
            ("dragons", "lobby_getBingoJackpotWinnersInfo") => {
                Some(DragonsLobbyGetBingoJackpotWinnersInfoParams::schema())
            }
            ("dragons", "lobby_getLootboxJackpotWinnersInfo") => {
                Some(DragonsLobbyGetLootboxJackpotWinnersInfoParams::schema())
            }
            ("dragons", "lobby_getLootboxJackpotPool") => {
                Some(DragonsLobbyGetLootboxJackpotPoolParams::schema())
            }
            ("dragons", "lobby_getPreviousTournamentStats") => {
                Some(DragonsLobbyGetPreviousTournamentStatsParams::schema())
            }
            ("dragons", "lobby_getSoftLeaderboardStats") => {
                Some(DragonsLobbyGetSoftLeaderboardStatsParams::schema())
            }
            ("dragons", "lobby_getTournamentLeaderboardStats") => {
                Some(DragonsLobbyGetTournamentLeaderboardStatsParams::schema())
            }
            ("dragons", "lobby_getLootboxesInfo") => {
                Some(DragonsLobbyGetLootboxesInfoParams::schema())
            }
            ("dragons", "lobby_getUsualLootboxTime") => {
                Some(DragonsLobbyGetUsualLootboxTimeParams::schema())
            }
            ("dragons", "lobby_openLootbox") => Some(DragonsLobbyOpenLootboxParams::schema()),
            ("dragons", "lobby_getAvailableMembershipAmount") => {
                Some(DragonsLobbyGetAvailableMembershipAmountParams::schema())
            }
            ("dragons", "lobby_getOffersAndBonuses") => {
                Some(DragonsLobbyGetOffersAndBonusesParams::schema())
            }
            ("dragons", "lobby_refuseOffer") => Some(DragonsLobbyRefuseOfferParams::schema()),
            ("dragons", "lobby_saveTutorialProgress") => {
                Some(DragonsLobbySaveTutorialProgressParams::schema())
            }
            ("dragons", "lobby_saveUsersProgress") => {
                Some(DragonsLobbySaveUsersProgressParams::schema())
            }
            ("dragons", "lobby_getUsersReferrals") => {
                Some(DragonsLobbyGetUsersReferralsParams::schema())
            }
            ("dragons", "lobby_setReferrer") => Some(DragonsLobbySetReferrerParams::schema()),
            ("dragons", "lobby_getServerTime") => Some(DragonsLobbyGetServerTimeParams::schema()),
            ("dragons", "lobby_keepAlive") => Some(DragonsLobbyKeepAliveParams::schema()),
            ("dragons", "lobby_getSpinRewards") => Some(DragonsLobbyGetSpinRewardsParams::schema()),
            ("dragons", "lobby_useSpin") => Some(DragonsLobbyUseSpinParams::schema()),
            ("dragons", "lobby_completeTask") => Some(DragonsLobbyCompleteTaskParams::schema()),
            ("dragons", "lobby_getCurrentTasks") => {
                Some(DragonsLobbyGetCurrentTasksParams::schema())
            }
            ("dragons", "lobby_getUsersCompletedTasks") => {
                Some(DragonsLobbyGetUsersCompletedTasksParams::schema())
            }
            ("dragons", "lobby_userValidation") => Some(DragonsLobbyUserValidationParams::schema()),
            ("dragons", "lobby_getBadTransactions") => {
                Some(DragonsLobbyGetBadTransactionsParams::schema())
            }
            ("dragons", "lobby_getUserIdByTransactionId") => {
                Some(DragonsLobbyGetUserIdByTransactionIdParams::schema())
            }
            ("dragons", "lobby_getUsersInappPurchases") => {
                Some(DragonsLobbyGetUsersInappPurchasesParams::schema())
            }
            ("dragons", "lobby_resetDeviceId") => Some(DragonsLobbyResetDeviceIdParams::schema()),
            ("dragons", "lobby_unlinkAccount") => Some(DragonsLobbyUnlinkAccountParams::schema()),
            ("dragons", "lobby_getProfile") => Some(DragonsLobbyGetProfileParams::schema()),
            ("dragons", "lobby_getSoulPrice") => Some(DragonsLobbyGetSoulPriceParams::schema()),
            ("dragons", "lobby_purchaseValidation") => {
                Some(DragonsLobbyPurchaseValidationParams::schema())
            }
            ("dragons", "lobby_retrieveUsersProgress") => {
                Some(DragonsLobbyRetrieveUsersProgressParams::schema())
            }
            ("dragons", "lobby_shareScore") => Some(DragonsLobbyShareScoreParams::schema()),
            ("dragons", "lobby_setUserAppMetricaDeviceId") => {
                Some(DragonsLobbySetUserAppMetricaDeviceIdParams::schema())
            }
            ("dragons", "lobby_getJackpotWinnersInfo") => {
                Some(DragonsLobbyGetJackpotWinnersInfoParams::schema())
            }
            ("dragons", "lobby_getProfileInfo") => Some(DragonsLobbyGetProfileInfoParams::schema()),
            ("dragons", "lobby_applicationInitialization") => {
                Some(DragonsLobbyApplicationInitializationParams::schema())
            }
            ("email", "email_findAll") => Some(EmailEmailFindAllParams::schema()),
            ("email", "email_createCronJob") => Some(EmailEmailCreateCronJobParams::schema()),
            ("email", "email_createAndSend") => Some(EmailEmailCreateAndSendParams::schema()),
            ("email", "email_sendNotSentLetters") => {
                Some(EmailEmailSendNotSentLettersParams::schema())
            }
            ("email", "email_sendInProcessLetters") => {
                Some(EmailEmailSendInProcessLettersParams::schema())
            }
            ("gameBalancer", "balancerEndpoints_addBots") => {
                Some(GameBalancerBalancerEndpointsAddBotsParams::schema())
            }
            ("gameBalancer", "coefficientEndpoints_get") => {
                Some(GameBalancerCoefficientEndpointsGetParams::schema())
            }
            ("gameBalancer", "coefficientEndpoints_update") => {
                Some(GameBalancerCoefficientEndpointsUpdateParams::schema())
            }
            ("gameBalancer", "gameBalancer_addPlayerInSearch") => {
                Some(GameBalancerGameBalancerAddPlayerInSearchParams::schema())
            }
            ("gameBalancer", "gameBalancer_disconnectPlayer") => {
                Some(GameBalancerGameBalancerDisconnectPlayerParams::schema())
            }
            ("gameBalancer", "gameBalancer_deletePlayers") => {
                Some(GameBalancerGameBalancerDeletePlayersParams::schema())
            }
            ("gameBalancer", "gameBalancer_stopSearching") => {
                Some(GameBalancerGameBalancerStopSearchingParams::schema())
            }
            ("gameBalancer", "gameBalancer_gameStartedNotification") => {
                Some(GameBalancerGameBalancerGameStartedNotificationParams::schema())
            }
            ("gameBalancer", "gameBalancer_checkInNotification") => {
                Some(GameBalancerGameBalancerCheckInNotificationParams::schema())
            }
            ("google-play", "purchase_validateSubscription") => {
                Some(GooglePlayPurchaseValidateSubscriptionParams::schema())
            }
            ("images", "image_uploadImage") => Some(ImagesImageUploadImageParams::schema()),
            ("js-tests", "dragocatsBattleProcessEndpoints_startTest") => {
                Some(JsTestsDragocatsBattleProcessEndpointsStartTestParams::schema())
            }
            ("js-tests", "dragocatsBattleProcessEndpoints_stopTest") => {
                Some(JsTestsDragocatsBattleProcessEndpointsStopTestParams::schema())
            }
            ("js-tests", "dragocatsBattleProcessEndpoints_getTestState") => {
                Some(JsTestsDragocatsBattleProcessEndpointsGetTestStateParams::schema())
            }
            ("listeria-storage", "heroesEndpoints_getById") => {
                Some(ListeriaStorageHeroesEndpointsGetByIdParams::schema())
            }
            ("listeria-storage", "heroesEndpoints_levelUp") => {
                Some(ListeriaStorageHeroesEndpointsLevelUpParams::schema())
            }
            ("listeria-storage", "heroesEndpoints_giveFreeExperienceToHero") => {
                Some(ListeriaStorageHeroesEndpointsGiveFreeExperienceToHeroParams::schema())
            }
            ("listeria-storage", "heroesEndpoints_getHeroesListByUserId") => {
                Some(ListeriaStorageHeroesEndpointsGetHeroesListByUserIdParams::schema())
            }
            ("listeria-storage", "heroesEndpoints_statsUpdatedNotification") => {
                Some(ListeriaStorageHeroesEndpointsStatsUpdatedNotificationParams::schema())
            }
            ("listeria-storage", "heroesEndpoints_heroAddedNotification") => {
                Some(ListeriaStorageHeroesEndpointsHeroAddedNotificationParams::schema())
            }
            ("listeria-storage", "inventoryEndpoints_getById") => {
                Some(ListeriaStorageInventoryEndpointsGetByIdParams::schema())
            }
            ("listeria-storage", "inventoryEndpoints_getEquipmentScrollsCount") => {
                Some(ListeriaStorageInventoryEndpointsGetEquipmentScrollsCountParams::schema())
            }
            ("listeria-storage", "inventoryEndpoints_getHeroScrollsCount") => {
                Some(ListeriaStorageInventoryEndpointsGetHeroScrollsCountParams::schema())
            }
            ("listeria-storage", "inventoryEndpoints_getLootboxesList") => {
                Some(ListeriaStorageInventoryEndpointsGetLootboxesListParams::schema())
            }
            ("listeria-storage", "inventoryEndpoints_openLootbox") => {
                Some(ListeriaStorageInventoryEndpointsOpenLootboxParams::schema())
            }
            ("listeria-storage", "inventoryEndpoints_addedUnequippableItemNotification") => Some(
                ListeriaStorageInventoryEndpointsAddedUnequippableItemNotificationParams::schema(),
            ),
            ("listeria-storage", "inventoryEndpoints_getItemsList") => {
                Some(ListeriaStorageInventoryEndpointsGetItemsListParams::schema())
            }
            ("listeria-storage", "inventoryEndpoints_levelUp") => {
                Some(ListeriaStorageInventoryEndpointsLevelUpParams::schema())
            }
            ("lobby", "equipmentEndpoints_equipItem") => {
                Some(LobbyEquipmentEndpointsEquipItemParams::schema())
            }
            ("lobby", "equipmentEndpoints_unequipItem") => {
                Some(LobbyEquipmentEndpointsUnequipItemParams::schema())
            }
            ("lobby", "equipmentEndpoints_getItemByItemIdAndUserId") => {
                Some(LobbyEquipmentEndpointsGetItemByItemIdAndUserIdParams::schema())
            }
            ("lobby", "equipmentEndpoints_updatedItemNotification") => {
                Some(LobbyEquipmentEndpointsUpdatedItemNotificationParams::schema())
            }
            ("lobby", "equipmentEndpoints_addedItemNotification") => {
                Some(LobbyEquipmentEndpointsAddedItemNotificationParams::schema())
            }
            ("lobby", "equipmentEndpoints_unEquipItemFromAllHeroes") => {
                Some(LobbyEquipmentEndpointsUnEquipItemFromAllHeroesParams::schema())
            }
            ("lobby", "heroesEndpoints_getHeroDTOAttributes") => {
                Some(LobbyHeroesEndpointsGetHeroDtoAttributesParams::schema())
            }
            ("lobby", "heroesEndpoints_getEquippedItems") => {
                Some(LobbyHeroesEndpointsGetEquippedItemsParams::schema())
            }
            ("lobby", "heroesEndpoints_equipHero") => {
                Some(LobbyHeroesEndpointsEquipHeroParams::schema())
            }
            ("lobby", "heroesEndpoints_getHeroesList") => {
                Some(LobbyHeroesEndpointsGetHeroesListParams::schema())
            }
            ("lobby", "heroesEndpoints_getActiveHeroId") => {
                Some(LobbyHeroesEndpointsGetActiveHeroIdParams::schema())
            }
            ("lobby", "heroesEndpoints_heroAddedNotification") => {
                Some(LobbyHeroesEndpointsHeroAddedNotificationParams::schema())
            }
            ("lobby", "lobby_startGame") => Some(LobbyLobbyStartGameParams::schema()),
            ("lobby", "regions_get") => Some(LobbyRegionsGetParams::schema()),
            ("lobby", "regions_set") => Some(LobbyRegionsSetParams::schema()),
            ("lobby", "regions_getList") => Some(LobbyRegionsGetListParams::schema()),
            ("lobby", "settings_get") => Some(LobbySettingsGetParams::schema()),
            ("lobby", "settings_set") => Some(LobbySettingsSetParams::schema()),
            ("lobby", "statsEndpoints_getLevelUpOptionsByPersonalType") => {
                Some(LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeParams::schema())
            }
            ("lobby", "statsEndpoints_getStatsOptionsByPersonalType") => {
                Some(LobbyStatsEndpointsGetStatsOptionsByPersonalTypeParams::schema())
            }
            ("lobby", "statsOptionsEndpoints_createOrUpdate") => {
                Some(LobbyStatsOptionsEndpointsCreateOrUpdateParams::schema())
            }
            ("lobby", "statsOptionsEndpoints_getByPersonalType") => {
                Some(LobbyStatsOptionsEndpointsGetByPersonalTypeParams::schema())
            }
            ("lobby", "statsOptionsEndpoints_getPersonalTypesList") => {
                Some(LobbyStatsOptionsEndpointsGetPersonalTypesListParams::schema())
            }
            ("lobby", "user_getUserData") => Some(LobbyUserGetUserDataParams::schema()),
            ("lobby", "user_updateUsername") => Some(LobbyUserUpdateUsernameParams::schema()),
            ("lobby", "user_getLeaderBoard") => Some(LobbyUserGetLeaderBoardParams::schema()),
            ("lobby", "user_updateProfileImage") => {
                Some(LobbyUserUpdateProfileImageParams::schema())
            }
            ("lootboxes", "lootboxes_openLootbox") => {
                Some(LootboxesLootboxesOpenLootboxParams::schema())
            }
            ("lootboxes", "lootboxes_getLootboxesInfo") => {
                Some(LootboxesLootboxesGetLootboxesInfoParams::schema())
            }
            ("lootboxes", "lootboxes_getUsualLootboxTime") => {
                Some(LootboxesLootboxesGetUsualLootboxTimeParams::schema())
            }
            ("market", "items_getPrice") => Some(MarketItemsGetPriceParams::schema()),
            ("market", "items_getLootboxesList") => {
                Some(MarketItemsGetLootboxesListParams::schema())
            }
            ("market-place", "marketEndpoints_getByFilter") => {
                Some(MarketPlaceMarketEndpointsGetByFilterParams::schema())
            }
            ("market-place", "marketEndpoints_getSimilar") => {
                Some(MarketPlaceMarketEndpointsGetSimilarParams::schema())
            }
            ("market-place", "marketPlace_buyItem") => {
                Some(MarketPlaceMarketPlaceBuyItemParams::schema())
            }
            ("market-place", "marketPlace_unlockItem") => {
                Some(MarketPlaceMarketPlaceUnlockItemParams::schema())
            }
            ("market-place", "marketPlace_getItemById") => {
                Some(MarketPlaceMarketPlaceGetItemByIdParams::schema())
            }
            ("market-place", "marketPlace_getBoughtItems") => {
                Some(MarketPlaceMarketPlaceGetBoughtItemsParams::schema())
            }
            ("market-place", "marketPlace_getItemsByFilter") => {
                Some(MarketPlaceMarketPlaceGetItemsByFilterParams::schema())
            }
            ("market-place", "marketPlace_lockItem") => {
                Some(MarketPlaceMarketPlaceLockItemParams::schema())
            }
            ("market-place", "marketPlace_cancelSale") => {
                Some(MarketPlaceMarketPlaceCancelSaleParams::schema())
            }
            ("market-place", "marketPlace_getCanceledItems") => {
                Some(MarketPlaceMarketPlaceGetCanceledItemsParams::schema())
            }
            ("market-place", "marketPlace_addToMarketPlace") => {
                Some(MarketPlaceMarketPlaceAddToMarketPlaceParams::schema())
            }
            ("near-adapter", "contract_isEnoughBalanceOnWithdrawWallet") => {
                Some(NearAdapterContractIsEnoughBalanceOnWithdrawWalletParams::schema())
            }
            ("near-adapter", "contract_callWithdraw") => {
                Some(NearAdapterContractCallWithdrawParams::schema())
            }
            ("near-adapter", "contract_callTransfer") => {
                Some(NearAdapterContractCallTransferParams::schema())
            }
            ("near-adapter", "contract_callMintNft") => {
                Some(NearAdapterContractCallMintNftParams::schema())
            }
            ("near-adapter", "contract_callBurnNft") => {
                Some(NearAdapterContractCallBurnNftParams::schema())
            }
            ("near-adapter", "contract_callNftLock") => {
                Some(NearAdapterContractCallNftLockParams::schema())
            }
            ("near-adapter", "contract_callNftUnlock") => {
                Some(NearAdapterContractCallNftUnlockParams::schema())
            }
            ("near-adapter", "contract_callNftUnlockAndTransfer") => {
                Some(NearAdapterContractCallNftUnlockAndTransferParams::schema())
            }
            ("near-adapter", "wallet_getMy") => Some(NearAdapterWalletGetMyParams::schema()),
            ("near-adapter", "wallet_getMyNft") => Some(NearAdapterWalletGetMyNftParams::schema()),
            ("near-adapter", "wallet_getUserByAccountId") => {
                Some(NearAdapterWalletGetUserByAccountIdParams::schema())
            }
            ("notifications", "notifications_createNew") => {
                Some(NotificationsNotificationsCreateNewParams::schema())
            }
            ("orchestrator", "balance_increaseUserBalance") => {
                Some(OrchestratorBalanceIncreaseUserBalanceParams::schema())
            }
            ("orchestrator", "balance_userBlockchainDeposit") => {
                Some(OrchestratorBalanceUserBlockchainDepositParams::schema())
            }
            ("orchestrator", "balance_decreaseUserBalance") => {
                Some(OrchestratorBalanceDecreaseUserBalanceParams::schema())
            }
            ("orchestrator", "battlePass_purchasePremium") => {
                Some(OrchestratorBattlePassPurchasePremiumParams::schema())
            }
            ("orchestrator", "battlePass_purchaseExperience") => {
                Some(OrchestratorBattlePassPurchaseExperienceParams::schema())
            }
            ("orchestrator", "battlePass_generateProduct") => {
                Some(OrchestratorBattlePassGenerateProductParams::schema())
            }
            ("orchestrator", "bingo_reward") => Some(OrchestratorBingoRewardParams::schema()),
            ("orchestrator", "lootbox_open") => Some(OrchestratorLootboxOpenParams::schema()),
            ("orchestrator", "lootbox_purchase") => {
                Some(OrchestratorLootboxPurchaseParams::schema())
            }
            ("orchestrator", "marketPlace_purchaseItem") => {
                Some(OrchestratorMarketPlacePurchaseItemParams::schema())
            }
            ("orchestrator", "product_burn") => Some(OrchestratorProductBurnParams::schema()),
            ("orchestrator", "purchase_purchaseProduct") => {
                Some(OrchestratorPurchasePurchaseProductParams::schema())
            }
            ("orchestrator", "wallet_getMy") => Some(OrchestratorWalletGetMyParams::schema()),
            ("orchestrator", "wallet_getByUserId") => {
                Some(OrchestratorWalletGetByUserIdParams::schema())
            }
            ("product-factory", "generator_generate") => {
                Some(ProductFactoryGeneratorGenerateParams::schema())
            }
            ("product-factory", "lootbox_open") => Some(ProductFactoryLootboxOpenParams::schema()),
            ("product-factory", "products_getInfoByProductId") => {
                Some(ProductFactoryProductsGetInfoByProductIdParams::schema())
            }
            ("productFactory", "lootboxType_add") => {
                Some(ProductFactoryLootboxTypeAddParams::schema())
            }
            ("productFactory", "lootboxType_getAll") => {
                Some(ProductFactoryLootboxTypeGetAllParams::schema())
            }
            ("productFactory", "lootboxType_updateByLootboxId") => {
                Some(ProductFactoryLootboxTypeUpdateByLootboxIdParams::schema())
            }
            ("productFactory", "lootboxType_getByLootboxId") => {
                Some(ProductFactoryLootboxTypeGetByLootboxIdParams::schema())
            }
            ("productFactory", "product_markHeroAsNFT") => {
                Some(ProductFactoryProductMarkHeroAsNftParams::schema())
            }
            ("productFactory", "product_addLootbox") => {
                Some(ProductFactoryProductAddLootboxParams::schema())
            }
            ("productFactory", "product_lootboxOpenedNotification") => {
                Some(ProductFactoryProductLootboxOpenedNotificationParams::schema())
            }
            ("productFactory", "productType_add") => {
                Some(ProductFactoryProductTypeAddParams::schema())
            }
            ("productFactory", "productType_isExist") => {
                Some(ProductFactoryProductTypeIsExistParams::schema())
            }
            ("productFactory", "productType_update") => {
                Some(ProductFactoryProductTypeUpdateParams::schema())
            }
            ("productFactory", "productType_delete") => {
                Some(ProductFactoryProductTypeDeleteParams::schema())
            }
            ("productFactory", "productType_getAll") => {
                Some(ProductFactoryProductTypeGetAllParams::schema())
            }
            ("productFactory", "productType_get") => {
                Some(ProductFactoryProductTypeGetParams::schema())
            }
            ("productFactory", "productType_getAllByType") => {
                Some(ProductFactoryProductTypeGetAllByTypeParams::schema())
            }
            ("productFactory", "productType_getAllByRarity") => {
                Some(ProductFactoryProductTypeGetAllByRarityParams::schema())
            }
            ("productFactory", "productType_getHash") => {
                Some(ProductFactoryProductTypeGetHashParams::schema())
            }
            ("productFactory", "productType_getAttributeListByPersonalType") => {
                Some(ProductFactoryProductTypeGetAttributeListByPersonalTypeParams::schema())
            }
            ("promo", "codes_useCode") => Some(PromoCodesUseCodeParams::schema()),
            ("promo", "codes_createCode") => Some(PromoCodesCreateCodeParams::schema()),
            ("promo", "codes_getDataByCodeStrict") => {
                Some(PromoCodesGetDataByCodeStrictParams::schema())
            }
            ("promo", "codes_getListStrict") => Some(PromoCodesGetListStrictParams::schema()),
            ("promo", "codes_deleteCode") => Some(PromoCodesDeleteCodeParams::schema()),
            ("purchase", "balance_increaseUserBalance") => {
                Some(PurchaseBalanceIncreaseUserBalanceParams::schema())
            }
            ("purchase", "balance_decreaseUserBalance") => {
                Some(PurchaseBalanceDecreaseUserBalanceParams::schema())
            }
            ("purchase", "balance_userBalanceChangedNotification") => {
                Some(PurchaseBalanceUserBalanceChangedNotificationParams::schema())
            }
            ("purchase", "product_purchaseLootbox") => {
                Some(PurchaseProductPurchaseLootboxParams::schema())
            }
            ("purchase", "product_getHeroList") => Some(PurchaseProductGetHeroListParams::schema()),
            ("purchase", "product_getEquipmentList") => {
                Some(PurchaseProductGetEquipmentListParams::schema())
            }
            ("purchase", "wallet_getByUserId") => Some(PurchaseWalletGetByUserIdParams::schema()),
            ("purchase", "wallet_getWalletByUserId") => {
                Some(PurchaseWalletGetWalletByUserIdParams::schema())
            }
            ("realis", "walletManager_getMyAddress") => {
                Some(RealisWalletManagerGetMyAddressParams::schema())
            }
            ("referral", "link_get") => Some(ReferralLinkGetParams::schema()),
            ("referral", "link_getMy") => Some(ReferralLinkGetMyParams::schema()),
            ("referral", "link_getAll") => Some(ReferralLinkGetAllParams::schema()),
            ("referral", "link_getAllMy") => Some(ReferralLinkGetAllMyParams::schema()),
            ("referral", "referral_getReferralInfoList") => {
                Some(ReferralReferralGetReferralInfoListParams::schema())
            }
            ("referral", "referral_addReferral") => {
                Some(ReferralReferralAddReferralParams::schema())
            }
            ("referral", "referral_setPotentialReferral") => {
                Some(ReferralReferralSetPotentialReferralParams::schema())
            }
            ("referral", "referral_getReferrals") => {
                Some(ReferralReferralGetReferralsParams::schema())
            }
            ("referral", "referral_getUserData") => {
                Some(ReferralReferralGetUserDataParams::schema())
            }
            ("referral", "reward_addReferralExpense") => {
                Some(ReferralRewardAddReferralExpenseParams::schema())
            }
            ("referral", "reward_makeRewardRequest") => {
                Some(ReferralRewardMakeRewardRequestParams::schema())
            }
            ("referral", "reward_getAvailableReward") => {
                Some(ReferralRewardGetAvailableRewardParams::schema())
            }
            ("refund", "balances_getAllMy") => Some(RefundBalancesGetAllMyParams::schema()),
            ("refund", "balances_getAll") => Some(RefundBalancesGetAllParams::schema()),
            ("refund", "balances_delete") => Some(RefundBalancesDeleteParams::schema()),
            ("refund", "balances_getAllUnavailable") => {
                Some(RefundBalancesGetAllUnavailableParams::schema())
            }
            ("refund", "balances_add") => Some(RefundBalancesAddParams::schema()),
            ("refund", "balances_getAllLockedFunds") => {
                Some(RefundBalancesGetAllLockedFundsParams::schema())
            }
            ("refund", "items_getAllMy") => Some(RefundItemsGetAllMyParams::schema()),
            ("refund", "items_getAll") => Some(RefundItemsGetAllParams::schema()),
            ("refund", "items_delete") => Some(RefundItemsDeleteParams::schema()),
            ("refund", "items_isAvailable") => Some(RefundItemsIsAvailableParams::schema()),
            ("refund", "items_getAllUnavailable") => {
                Some(RefundItemsGetAllUnavailableParams::schema())
            }
            ("refund", "items_add") => Some(RefundItemsAddParams::schema()),
            ("soul-adapter", "wallet_getUserIdByAddress") => {
                Some(SoulAdapterWalletGetUserIdByAddressParams::schema())
            }
            ("soul-adapter", "wallet_getMyWallet") => {
                Some(SoulAdapterWalletGetMyWalletParams::schema())
            }
            ("soul-adapter", "wallet_processTransactionByHash") => {
                Some(SoulAdapterWalletProcessTransactionByHashParams::schema())
            }
            ("soul-adapter", "wallet_getBalanceByAddress") => {
                Some(SoulAdapterWalletGetBalanceByAddressParams::schema())
            }
            ("status", "config_add") => Some(StatusConfigAddParams::schema()),
            ("status", "config_delete") => Some(StatusConfigDeleteParams::schema()),
            ("status", "config_disable") => Some(StatusConfigDisableParams::schema()),
            ("status", "config_update") => Some(StatusConfigUpdateParams::schema()),
            ("status", "config_getAll") => Some(StatusConfigGetAllParams::schema()),
            ("status", "config_getMembershipInfo") => {
                Some(StatusConfigGetMembershipInfoParams::schema())
            }
            ("status", "config_updatePriorityIndex") => {
                Some(StatusConfigUpdatePriorityIndexParams::schema())
            }
            ("status", "config_getList") => Some(StatusConfigGetListParams::schema()),
            ("status", "config_getAllForPurchase") => {
                Some(StatusConfigGetAllForPurchaseParams::schema())
            }
            ("status", "membership_update") => Some(StatusMembershipUpdateParams::schema()),
            ("status", "membership_addAwardMembership") => {
                Some(StatusMembershipAddAwardMembershipParams::schema())
            }
            ("status", "membership_getInfo") => Some(StatusMembershipGetInfoParams::schema()),
            ("status", "membership_changeMembershipGame") => {
                Some(StatusMembershipChangeMembershipGameParams::schema())
            }
            ("status", "membership_disable") => Some(StatusMembershipDisableParams::schema()),
            ("status", "membership_getAllActive") => {
                Some(StatusMembershipGetAllActiveParams::schema())
            }
            ("status", "membership_getAllMyActive") => {
                Some(StatusMembershipGetAllMyActiveParams::schema())
            }
            ("status", "membership_purchaseMembershipFromSite") => {
                Some(StatusMembershipPurchaseMembershipFromSiteParams::schema())
            }
            ("status", "membership_getUserByGPA") => {
                Some(StatusMembershipGetUserByGpaParams::schema())
            }
            ("status", "membership_checkUserHasActiveStatus") => {
                Some(StatusMembershipCheckUserHasActiveStatusParams::schema())
            }
            ("task", "task_completeTask") => Some(TaskTaskCompleteTaskParams::schema()),
            ("task", "task_getCurrentTasks") => Some(TaskTaskGetCurrentTasksParams::schema()),
            ("task", "task_getUsersCompletedTasks") => {
                Some(TaskTaskGetUsersCompletedTasksParams::schema())
            }
            ("transactions", "balance_getBalanceByUserId") => {
                Some(TransactionsBalanceGetBalanceByUserIdParams::schema())
            }
            ("transactions", "balance_getBalancesByUserId") => {
                Some(TransactionsBalanceGetBalancesByUserIdParams::schema())
            }
            ("transactions", "balance_getBalancesByUserIdAsArray") => {
                Some(TransactionsBalanceGetBalancesByUserIdAsArrayParams::schema())
            }
            ("transactions", "balance_increaseBalanceByUserId") => {
                Some(TransactionsBalanceIncreaseBalanceByUserIdParams::schema())
            }
            ("transactions", "balance_decreaseBalanceByUserId") => {
                Some(TransactionsBalanceDecreaseBalanceByUserIdParams::schema())
            }
            ("transactions", "balance_getBalancesInUsd") => {
                Some(TransactionsBalanceGetBalancesInUsdParams::schema())
            }
            ("transactions", "balance_convert") => Some(TransactionsBalanceConvertParams::schema()),
            ("transactions", "balance_getMyBalancesWithRounding") => {
                Some(TransactionsBalanceGetMyBalancesWithRoundingParams::schema())
            }
            ("transactions", "balance_getAll") => Some(TransactionsBalanceGetAllParams::schema()),
            ("transactions", "balance_getAllMy") => {
                Some(TransactionsBalanceGetAllMyParams::schema())
            }
            ("transactions", "balance_getMyNumOfTransactions") => {
                Some(TransactionsBalanceGetMyNumOfTransactionsParams::schema())
            }
            ("transactions", "balance_getAllCreditTransactionList") => {
                Some(TransactionsBalanceGetAllCreditTransactionListParams::schema())
            }
            ("transactions", "balance_getUserBalances") => {
                Some(TransactionsBalanceGetUserBalancesParams::schema())
            }
            ("transactions", "balance_getWithFilter") => {
                Some(TransactionsBalanceGetWithFilterParams::schema())
            }
            ("transactions", "balance_getNumOfTransactions") => {
                Some(TransactionsBalanceGetNumOfTransactionsParams::schema())
            }
            ("transactions", "balance_updateTransactionHashAndBlockId") => {
                Some(TransactionsBalanceUpdateTransactionHashAndBlockIdParams::schema())
            }
            ("transactions", "balance_updateTransactionDataFromBlockchain") => {
                Some(TransactionsBalanceUpdateTransactionDataFromBlockchainParams::schema())
            }
            ("transactions", "balance_getListWithPagination") => {
                Some(TransactionsBalanceGetListWithPaginationParams::schema())
            }
            ("transactions", "balance_deleteBalanceByUserId") => {
                Some(TransactionsBalanceDeleteBalanceByUserIdParams::schema())
            }
            ("transactions", "balance_getAllLisSum") => {
                Some(TransactionsBalanceGetAllLisSumParams::schema())
            }
            ("transactions", "balance_adminIncreaseBalance") => {
                Some(TransactionsBalanceAdminIncreaseBalanceParams::schema())
            }
            ("transactions", "balance_adminDecreaseBalance") => {
                Some(TransactionsBalanceAdminDecreaseBalanceParams::schema())
            }
            ("transactions", "balance_getTestnetLis") => {
                Some(TransactionsBalanceGetTestnetLisParams::schema())
            }
            ("transactions", "balance_getNumWithFilter") => {
                Some(TransactionsBalanceGetNumWithFilterParams::schema())
            }
            ("transactions", "balance_checkTransactionForCurrency") => {
                Some(TransactionsBalanceCheckTransactionForCurrencyParams::schema())
            }
            ("transactions", "balance_checkMyTransactionForCurrency") => {
                Some(TransactionsBalanceCheckMyTransactionForCurrencyParams::schema())
            }
            ("transactions", "registryProduct_addProduct") => {
                Some(TransactionsRegistryProductAddProductParams::schema())
            }
            ("transactions", "registryProduct_addProductHash") => {
                Some(TransactionsRegistryProductAddProductHashParams::schema())
            }
            ("transactions", "registryProduct_getPersonalTypesByUser") => {
                Some(TransactionsRegistryProductGetPersonalTypesByUserParams::schema())
            }
            ("transactions", "registryProduct_deleteProductByUserId") => {
                Some(TransactionsRegistryProductDeleteProductByUserIdParams::schema())
            }
            ("transactions", "registryProduct_burnProduct") => {
                Some(TransactionsRegistryProductBurnProductParams::schema())
            }
            ("transactions", "registryProduct_updateProductOwner") => {
                Some(TransactionsRegistryProductUpdateProductOwnerParams::schema())
            }
            ("user", "profile_changeNickname") => Some(UserProfileChangeNicknameParams::schema()),
            ("user", "profile_getProfile") => Some(UserProfileGetProfileParams::schema()),
            ("user", "profile_getMyProfile") => Some(UserProfileGetMyProfileParams::schema()),
            ("user", "profile_getAllProfiles") => Some(UserProfileGetAllProfilesParams::schema()),
            ("user", "profile_unsetNotice") => Some(UserProfileUnsetNoticeParams::schema()),
            ("user", "profile_setSuspicious") => Some(UserProfileSetSuspiciousParams::schema()),
            ("user", "profile_unsetSuspicious") => Some(UserProfileUnsetSuspiciousParams::schema()),
            ("user", "profile_banProfile") => Some(UserProfileBanProfileParams::schema()),
            ("user", "profile_unBan") => Some(UserProfileUnBanParams::schema()),
            ("user", "profile_setMailingSubscriptionStatus") => {
                Some(UserProfileSetMailingSubscriptionStatusParams::schema())
            }
            ("user", "profile_unsubscribeFromNewsletter") => {
                Some(UserProfileUnsubscribeFromNewsletterParams::schema())
            }
            ("user", "profile_getAllUsersSubscribedToMailing") => {
                Some(UserProfileGetAllUsersSubscribedToMailingParams::schema())
            }
            ("user", "profile_getBanInfo") => Some(UserProfileGetBanInfoParams::schema()),
            ("user", "profile_getMyBanInfo") => Some(UserProfileGetMyBanInfoParams::schema()),
            ("user", "profile_getNicknameChangePrice") => {
                Some(UserProfileGetNicknameChangePriceParams::schema())
            }
            ("user", "profile_getUserByParams") => Some(UserProfileGetUserByParamsParams::schema()),
            ("user", "profile_getMyNickname") => Some(UserProfileGetMyNicknameParams::schema()),
            ("user", "profile_getUserNickname") => Some(UserProfileGetUserNicknameParams::schema()),
            ("user", "profile_getNicknamesByUserIds") => {
                Some(UserProfileGetNicknamesByUserIdsParams::schema())
            }
            ("user", "profile_deleteUser") => Some(UserProfileDeleteUserParams::schema()),
            ("user", "profile_setNotice") => Some(UserProfileSetNoticeParams::schema()),
            ("user", "profile_changeEmail") => Some(UserProfileChangeEmailParams::schema()),
            ("user", "profile_isEmailExists") => Some(UserProfileIsEmailExistsParams::schema()),
            ("user", "profile_getUserIdByEmail") => {
                Some(UserProfileGetUserIdByEmailParams::schema())
            }
            ("user", "profile_deleteUserRecord") => {
                Some(UserProfileDeleteUserRecordParams::schema())
            }
            ("user", "profile_getNum") => Some(UserProfileGetNumParams::schema()),
            ("user", "profile_getMyProfileForBytes") => {
                Some(UserProfileGetMyProfileForBytesParams::schema())
            }
            ("user", "profile_getTestData") => Some(UserProfileGetTestDataParams::schema()),
            ("user", "profile_getCountActives") => Some(UserProfileGetCountActivesParams::schema()),
            ("user", "status_delete") => Some(UserStatusDeleteParams::schema()),
            ("user", "status_get") => Some(UserStatusGetParams::schema()),
            ("withdraw", "approval_getAllMyTransactions") => {
                Some(WithdrawApprovalGetAllMyTransactionsParams::schema())
            }
            ("withdraw", "approval_listUnapproved") => {
                Some(WithdrawApprovalListUnapprovedParams::schema())
            }
            ("withdraw", "approval_approve") => Some(WithdrawApprovalApproveParams::schema()),
            ("withdraw", "approval_deny") => Some(WithdrawApprovalDenyParams::schema()),
            ("withdraw", "attempt_tryNew") => Some(WithdrawAttemptTryNewParams::schema()),
            _ => None,
        }
    }
    pub fn get_returns_schema(agent: &str, method: &str) -> Option<Value> {
        match (agent, method) {
            ("achievements", "achievement_achievementComplete") => {
                Some(AchievementsAchievementAchievementCompleteReturns::schema())
            }
            ("achievements", "achievement_getAllAchievements") => {
                Some(AchievementsAchievementGetAllAchievementsReturns::schema())
            }
            ("achievements", "achievement_getUsersAchievements") => {
                Some(AchievementsAchievementGetUsersAchievementsReturns::schema())
            }
            ("admin", "action_undo") => Some(AdminActionUndoReturns::schema()),
            ("admin", "action_getActionList") => Some(AdminActionGetActionListReturns::schema()),
            ("admin", "action_deleteByActionId") => {
                Some(AdminActionDeleteByActionIdReturns::schema())
            }
            ("admin", "action_getAllByFilterList") => {
                Some(AdminActionGetAllByFilterListReturns::schema())
            }
            ("admin", "confirmation_addSolutionAdmin") => {
                Some(AdminConfirmationAddSolutionAdminReturns::schema())
            }
            ("admin", "confirmation_deleteAction") => {
                Some(AdminConfirmationDeleteActionReturns::schema())
            }
            ("admin", "confirmation_getAllActions") => {
                Some(AdminConfirmationGetAllActionsReturns::schema())
            }
            ("admin", "confirmation_getNotConfirmedActions") => {
                Some(AdminConfirmationGetNotConfirmedActionsReturns::schema())
            }
            ("admin", "confirmation_add") => Some(AdminConfirmationAddReturns::schema()),
            ("admin", "confirmation_addSolutionAdminForMany") => {
                Some(AdminConfirmationAddSolutionAdminForManyReturns::schema())
            }
            ("admin", "mailTemplate_create") => Some(AdminMailTemplateCreateReturns::schema()),
            ("admin", "mailTemplate_delete") => Some(AdminMailTemplateDeleteReturns::schema()),
            ("admin", "mailTemplate_getByKey") => Some(AdminMailTemplateGetByKeyReturns::schema()),
            ("admin", "mailTemplate_change") => Some(AdminMailTemplateChangeReturns::schema()),
            ("admin", "mailTemplate_getAll") => Some(AdminMailTemplateGetAllReturns::schema()),
            ("admin", "option_set") => Some(AdminOptionSetReturns::schema()),
            ("admin", "option_unset") => Some(AdminOptionUnsetReturns::schema()),
            ("admin", "option_get") => Some(AdminOptionGetReturns::schema()),
            ("admin", "option_getList") => Some(AdminOptionGetListReturns::schema()),
            ("admin", "option_getByScope") => Some(AdminOptionGetByScopeReturns::schema()),
            ("admin", "option_getAllByFilter") => Some(AdminOptionGetAllByFilterReturns::schema()),
            ("admin", "option_update") => Some(AdminOptionUpdateReturns::schema()),
            ("admin", "option_updateAll") => Some(AdminOptionUpdateAllReturns::schema()),
            ("admin", "permission_add") => Some(AdminPermissionAddReturns::schema()),
            ("admin", "permission_delete") => Some(AdminPermissionDeleteReturns::schema()),
            ("admin", "permission_updatePermissions") => {
                Some(AdminPermissionUpdatePermissionsReturns::schema())
            }
            ("admin", "role_add") => Some(AdminRoleAddReturns::schema()),
            ("admin", "role_delete") => Some(AdminRoleDeleteReturns::schema()),
            ("admin", "role_getUsersByRole") => Some(AdminRoleGetUsersByRoleReturns::schema()),
            ("admin", "role_getPermissionsByRoleName") => {
                Some(AdminRoleGetPermissionsByRoleNameReturns::schema())
            }
            ("admin", "role_getAllRoles") => Some(AdminRoleGetAllRolesReturns::schema()),
            ("admin", "role_getRoleByName") => Some(AdminRoleGetRoleByNameReturns::schema()),
            ("admin", "userRole_add") => Some(AdminUserRoleAddReturns::schema()),
            ("admin", "userRole_getAll") => Some(AdminUserRoleGetAllReturns::schema()),
            ("admin", "userRole_deleteUserRole") => {
                Some(AdminUserRoleDeleteUserRoleReturns::schema())
            }
            ("admin", "userRole_getAllWithNickname") => {
                Some(AdminUserRoleGetAllWithNicknameReturns::schema())
            }
            ("admin", "userRole_getMyRole") => Some(AdminUserRoleGetMyRoleReturns::schema()),
            ("admin", "userRole_getAllUsersWithNickname") => {
                Some(AdminUserRoleGetAllUsersWithNicknameReturns::schema())
            }
            ("admin", "userRole_isAdmin") => Some(AdminUserRoleIsAdminReturns::schema()),
            ("analytics", "analytics_send") => Some(AnalyticsAnalyticsSendReturns::schema()),
            ("auth", "admin_login") => Some(AuthAdminLoginReturns::schema()),
            ("auth", "admin_addUserRole") => Some(AuthAdminAddUserRoleReturns::schema()),
            ("auth", "admin_deleteUserRole") => Some(AuthAdminDeleteUserRoleReturns::schema()),
            ("auth", "admin_updateUserRole") => Some(AuthAdminUpdateUserRoleReturns::schema()),
            ("auth", "admin_getUserRole") => Some(AuthAdminGetUserRoleReturns::schema()),
            ("auth", "admin_getMyRole") => Some(AuthAdminGetMyRoleReturns::schema()),
            ("auth", "admin_getUsers") => Some(AuthAdminGetUsersReturns::schema()),
            ("auth", "auth_login") => Some(AuthAuthLoginReturns::schema()),
            ("auth", "auth_socialNetworkAuth") => Some(AuthAuthSocialNetworkAuthReturns::schema()),
            ("auth", "auth_changePassword") => Some(AuthAuthChangePasswordReturns::schema()),
            ("auth", "auth_sendRequestToResetPassword") => {
                Some(AuthAuthSendRequestToResetPasswordReturns::schema())
            }
            ("auth", "auth_resetPassword") => Some(AuthAuthResetPasswordReturns::schema()),
            ("auth", "auth_sendRequestToDeleteAccount") => {
                Some(AuthAuthSendRequestToDeleteAccountReturns::schema())
            }
            ("auth", "auth_removeAccount") => Some(AuthAuthRemoveAccountReturns::schema()),
            ("auth", "auth_getAccessTokenByRefresh") => {
                Some(AuthAuthGetAccessTokenByRefreshReturns::schema())
            }
            ("auth", "auth_getAllMyActiveSessions") => {
                Some(AuthAuthGetAllMyActiveSessionsReturns::schema())
            }
            ("auth", "auth_confirmPassword") => Some(AuthAuthConfirmPasswordReturns::schema()),
            ("auth", "auth_deleteMyUser") => Some(AuthAuthDeleteMyUserReturns::schema()),
            ("auth", "auth_getUserInfoByToken") => {
                Some(AuthAuthGetUserInfoByTokenReturns::schema())
            }
            ("auth", "auth_createRequestToConfirmEmail") => {
                Some(AuthAuthCreateRequestToConfirmEmailReturns::schema())
            }
            ("auth", "auth_confirmEmailByCode") => {
                Some(AuthAuthConfirmEmailByCodeReturns::schema())
            }
            ("auth", "auth_confirmEmailByHash") => {
                Some(AuthAuthConfirmEmailByHashReturns::schema())
            }
            ("auth", "auth_deleteEmailRequest") => {
                Some(AuthAuthDeleteEmailRequestReturns::schema())
            }
            ("auth", "auth_setPassword") => Some(AuthAuthSetPasswordReturns::schema()),
            ("auth", "auth_disableUser") => Some(AuthAuthDisableUserReturns::schema()),
            ("auth", "auth_resentConfirmationMail") => {
                Some(AuthAuthResentConfirmationMailReturns::schema())
            }
            ("auth", "auth_createRequestToChangeEmail") => {
                Some(AuthAuthCreateRequestToChangeEmailReturns::schema())
            }
            ("auth", "auth_changeMyEmail") => Some(AuthAuthChangeMyEmailReturns::schema()),
            ("auth", "auth_loginWithFacebook") => Some(AuthAuthLoginWithFacebookReturns::schema()),
            ("auth", "auth_logout") => Some(AuthAuthLogoutReturns::schema()),
            ("auth", "auth_emptyMethod") => Some(AuthAuthEmptyMethodReturns::schema()),
            ("auth", "auth_assignProviderAccountToDeviceId") => {
                Some(AuthAuthAssignProviderAccountToDeviceIdReturns::schema())
            }
            ("auth", "auth_resetUserProgress") => Some(AuthAuthResetUserProgressReturns::schema()),
            ("auth", "authDevice_getOrCreateInternalUserId") => {
                Some(AuthAuthDeviceGetOrCreateInternalUserIdReturns::schema())
            }
            ("auth", "authDevice_isNewClient") => Some(AuthAuthDeviceIsNewClientReturns::schema()),
            ("auth", "authDevice_isAuthorized") => {
                Some(AuthAuthDeviceIsAuthorizedReturns::schema())
            }
            ("auth", "authDevice_getClientInfo") => {
                Some(AuthAuthDeviceGetClientInfoReturns::schema())
            }
            ("auth", "authDevice_updateInternalIdByDeviceId") => {
                Some(AuthAuthDeviceUpdateInternalIdByDeviceIdReturns::schema())
            }
            ("auth", "authDevice_getClientStatusByInternalUserId") => {
                Some(AuthAuthDeviceGetClientStatusByInternalUserIdReturns::schema())
            }
            ("auth", "ban_ban") => Some(AuthBanBanReturns::schema()),
            ("auth", "ban_unBan") => Some(AuthBanUnBanReturns::schema()),
            ("auth", "ban_getUserBan") => Some(AuthBanGetUserBanReturns::schema()),
            ("auth", "instantMigration_initMigration") => {
                Some(AuthInstantMigrationInitMigrationReturns::schema())
            }
            ("auth", "instantMigration_stopMigration") => {
                Some(AuthInstantMigrationStopMigrationReturns::schema())
            }
            ("auth", "role_add") => Some(AuthRoleAddReturns::schema()),
            ("auth", "role_delete") => Some(AuthRoleDeleteReturns::schema()),
            ("auth", "role_update") => Some(AuthRoleUpdateReturns::schema()),
            ("auth", "role_getAllRoles") => Some(AuthRoleGetAllRolesReturns::schema()),
            ("auth", "role_getRoleByName") => Some(AuthRoleGetRoleByNameReturns::schema()),
            ("auth", "twoFactor_getSecret") => Some(AuthTwoFactorGetSecretReturns::schema()),
            ("auth", "twoFactor_verifyCode") => Some(AuthTwoFactorVerifyCodeReturns::schema()),
            ("auth", "twoFactor_disableTwoFA") => Some(AuthTwoFactorDisableTwoFaReturns::schema()),
            ("auth", "twoFactor_enableTwoFA") => Some(AuthTwoFactorEnableTwoFaReturns::schema()),
            ("auth", "twoFactor_isRequiredTwoFA") => {
                Some(AuthTwoFactorIsRequiredTwoFaReturns::schema())
            }
            ("auth", "twoFactor_getStatus") => Some(AuthTwoFactorGetStatusReturns::schema()),
            ("auth", "twoFactor_sendRequestToDeleteTwoFA") => {
                Some(AuthTwoFactorSendRequestToDeleteTwoFaReturns::schema())
            }
            ("auth", "twoFactor_deleteTwoFA") => Some(AuthTwoFactorDeleteTwoFaReturns::schema()),
            ("balances", "balances_freeExperienceAddedNotification") => {
                Some(BalancesBalancesFreeExperienceAddedNotificationReturns::schema())
            }
            ("balances", "balances_getSoftCurrencyBalance") => {
                Some(BalancesBalancesGetSoftCurrencyBalanceReturns::schema())
            }
            ("balances", "balances_increaseSoftCurrency") => {
                Some(BalancesBalancesIncreaseSoftCurrencyReturns::schema())
            }
            ("balances", "balances_decreaseSoftCurrency") => {
                Some(BalancesBalancesDecreaseSoftCurrencyReturns::schema())
            }
            ("balances", "balances_addFreeExperience") => {
                Some(BalancesBalancesAddFreeExperienceReturns::schema())
            }
            ("balances", "balances_useFreeExperience") => {
                Some(BalancesBalancesUseFreeExperienceReturns::schema())
            }
            ("balances", "balances_getBalancesByUserId") => {
                Some(BalancesBalancesGetBalancesByUserIdReturns::schema())
            }
            ("balances", "balances_getFreeExperienceBalance") => {
                Some(BalancesBalancesGetFreeExperienceBalanceReturns::schema())
            }
            ("battle-pass", "battlePassEndpoints_receiveReward") => {
                Some(BattlePassBattlePassEndpointsReceiveRewardReturns::schema())
            }
            ("battle-pass", "battlePassEndpoints_getBattlePassData") => {
                Some(BattlePassBattlePassEndpointsGetBattlePassDataReturns::schema())
            }
            ("battle-pass", "battlePassEndpoints_getCurrentBattlePassExperience") => {
                Some(BattlePassBattlePassEndpointsGetCurrentBattlePassExperienceReturns::schema())
            }
            ("battle-pass", "battlePassEndpoints_getBattlePassSeasonInfo") => {
                Some(BattlePassBattlePassEndpointsGetBattlePassSeasonInfoReturns::schema())
            }
            ("battle-pass", "battlePassEndpoints_getBattlePassPrices") => {
                Some(BattlePassBattlePassEndpointsGetBattlePassPricesReturns::schema())
            }
            ("battle-pass", "battlePassEndpoints_receiveOldBattlePassRewards") => {
                Some(BattlePassBattlePassEndpointsReceiveOldBattlePassRewardsReturns::schema())
            }
            ("battle-pass", "battlePassEndpoints_purchaseBattlePassExperienceToLevelup") => Some(
                BattlePassBattlePassEndpointsPurchaseBattlePassExperienceToLevelupReturns::schema(),
            ),
            ("battle-pass", "battlePassEndpoints_purchaseBattlePassPremium") => {
                Some(BattlePassBattlePassEndpointsPurchaseBattlePassPremiumReturns::schema())
            }
            ("binance-wallet", "wallet_createWallet") => {
                Some(BinanceWalletWalletCreateWalletReturns::schema())
            }
            ("binance-wallet", "wallet_getAddressByUserId") => {
                Some(BinanceWalletWalletGetAddressByUserIdReturns::schema())
            }
            ("binance-wallet", "wallet_getUserIdByAddress") => {
                Some(BinanceWalletWalletGetUserIdByAddressReturns::schema())
            }
            ("bingo", "bingo_getBingoData") => Some(BingoBingoGetBingoDataReturns::schema()),
            ("bingo", "bingo_craftBingoRecipe") => {
                Some(BingoBingoCraftBingoRecipeReturns::schema())
            }
            ("bingo", "bingo_getBingoJackpotPool") => {
                Some(BingoBingoGetBingoJackpotPoolReturns::schema())
            }
            ("bingo", "bingo_getBingoJackpotWinnersInfo") => {
                Some(BingoBingoGetBingoJackpotWinnersInfoReturns::schema())
            }
            ("bingo", "bingo_getCurrentBingoSeason") => {
                Some(BingoBingoGetCurrentBingoSeasonReturns::schema())
            }
            ("blog", "blog_create") => Some(BlogBlogCreateReturns::schema()),
            ("blog", "blog_update") => Some(BlogBlogUpdateReturns::schema()),
            ("blog", "blog_delete") => Some(BlogBlogDeleteReturns::schema()),
            ("blog", "blog_getAll") => Some(BlogBlogGetAllReturns::schema()),
            ("blog", "blog_get") => Some(BlogBlogGetReturns::schema()),
            ("blog", "blog_getByUrl") => Some(BlogBlogGetByUrlReturns::schema()),
            ("blog", "blog_getPinned") => Some(BlogBlogGetPinnedReturns::schema()),
            ("blog", "blog_getAllByFilter") => Some(BlogBlogGetAllByFilterReturns::schema()),
            ("blog", "blog_getAllWithCategoryList") => {
                Some(BlogBlogGetAllWithCategoryListReturns::schema())
            }
            ("blog", "category_create") => Some(BlogCategoryCreateReturns::schema()),
            ("blog", "category_update") => Some(BlogCategoryUpdateReturns::schema()),
            ("blog", "category_delete") => Some(BlogCategoryDeleteReturns::schema()),
            ("blog", "category_getAll") => Some(BlogCategoryGetAllReturns::schema()),
            ("blog", "category_get") => Some(BlogCategoryGetReturns::schema()),
            ("blog", "poll_add") => Some(BlogPollAddReturns::schema()),
            ("blog", "poll_get") => Some(BlogPollGetReturns::schema()),
            ("blog", "poll_getAll") => Some(BlogPollGetAllReturns::schema()),
            ("blog", "poll_update") => Some(BlogPollUpdateReturns::schema()),
            ("blog", "poll_delete") => Some(BlogPollDeleteReturns::schema()),
            ("blog", "vote_addVote") => Some(BlogVoteAddVoteReturns::schema()),
            ("blog", "vote_getAllVotesByPollId") => {
                Some(BlogVoteGetAllVotesByPollIdReturns::schema())
            }
            ("blog", "vote_isVoted") => Some(BlogVoteIsVotedReturns::schema()),
            ("cats", "lobby_achievementComplete") => {
                Some(CatsLobbyAchievementCompleteReturns::schema())
            }
            ("cats", "lobby_getAllAchievements") => {
                Some(CatsLobbyGetAllAchievementsReturns::schema())
            }
            ("cats", "lobby_getUsersAchievements") => {
                Some(CatsLobbyGetUsersAchievementsReturns::schema())
            }
            ("cats", "lobby_adventureMapLevelUp") => {
                Some(CatsLobbyAdventureMapLevelUpReturns::schema())
            }
            ("cats", "lobby_getAdventureMapStats") => {
                Some(CatsLobbyGetAdventureMapStatsReturns::schema())
            }
            ("cats", "lobby_getApplicationSettings") => {
                Some(CatsLobbyGetApplicationSettingsReturns::schema())
            }
            ("cats", "lobby_craftBingoRecipe") => Some(CatsLobbyCraftBingoRecipeReturns::schema()),
            ("cats", "lobby_getBingoData") => Some(CatsLobbyGetBingoDataReturns::schema()),
            ("cats", "lobby_getUsersCards") => Some(CatsLobbyGetUsersCardsReturns::schema()),
            ("cats", "lobby_upgradeCard") => Some(CatsLobbyUpgradeCardReturns::schema()),
            ("cats", "lobby_getClientVersion") => Some(CatsLobbyGetClientVersionReturns::schema()),
            ("cats", "lobby_getConfigById") => Some(CatsLobbyGetConfigByIdReturns::schema()),
            ("cats", "lobby_updateConfig") => Some(CatsLobbyUpdateConfigReturns::schema()),
            ("cats", "lobby_buyCat") => Some(CatsLobbyBuyCatReturns::schema()),
            ("cats", "lobby_buyScience") => Some(CatsLobbyBuyScienceReturns::schema()),
            ("cats", "lobby_buySkill") => Some(CatsLobbyBuySkillReturns::schema()),
            ("cats", "lobby_buyUsualLootbox") => Some(CatsLobbyBuyUsualLootboxReturns::schema()),
            ("cats", "lobby_getBingoJackpotPool") => {
                Some(CatsLobbyGetBingoJackpotPoolReturns::schema())
            }
            ("cats", "lobby_getBingoJackpotWinnersInfo") => {
                Some(CatsLobbyGetBingoJackpotWinnersInfoReturns::schema())
            }
            ("cats", "lobby_getLootboxJackpotWinnersInfo") => {
                Some(CatsLobbyGetLootboxJackpotWinnersInfoReturns::schema())
            }
            ("cats", "lobby_getLootboxJackpotPool") => {
                Some(CatsLobbyGetLootboxJackpotPoolReturns::schema())
            }
            ("cats", "lobby_getPreviousTournamentStats") => {
                Some(CatsLobbyGetPreviousTournamentStatsReturns::schema())
            }
            ("cats", "lobby_getSoftLeaderboardStats") => {
                Some(CatsLobbyGetSoftLeaderboardStatsReturns::schema())
            }
            ("cats", "lobby_getTournamentLeaderboardStats") => {
                Some(CatsLobbyGetTournamentLeaderboardStatsReturns::schema())
            }
            ("cats", "lobby_getLootboxesInfo") => Some(CatsLobbyGetLootboxesInfoReturns::schema()),
            ("cats", "lobby_getUsualLootboxTime") => {
                Some(CatsLobbyGetUsualLootboxTimeReturns::schema())
            }
            ("cats", "lobby_openLootbox") => Some(CatsLobbyOpenLootboxReturns::schema()),
            ("cats", "lobby_getAvailableMembershipAmount") => {
                Some(CatsLobbyGetAvailableMembershipAmountReturns::schema())
            }
            ("cats", "lobby_getOffersAndBonuses") => {
                Some(CatsLobbyGetOffersAndBonusesReturns::schema())
            }
            ("cats", "lobby_refuseOffer") => Some(CatsLobbyRefuseOfferReturns::schema()),
            ("cats", "lobby_saveTutorialProgress") => {
                Some(CatsLobbySaveTutorialProgressReturns::schema())
            }
            ("cats", "lobby_saveUsersProgress") => {
                Some(CatsLobbySaveUsersProgressReturns::schema())
            }
            ("cats", "lobby_getUsersReferrals") => {
                Some(CatsLobbyGetUsersReferralsReturns::schema())
            }
            ("cats", "lobby_setReferrer") => Some(CatsLobbySetReferrerReturns::schema()),
            ("cats", "lobby_getServerTime") => Some(CatsLobbyGetServerTimeReturns::schema()),
            ("cats", "lobby_keepAlive") => Some(CatsLobbyKeepAliveReturns::schema()),
            ("cats", "lobby_getSpinRewards") => Some(CatsLobbyGetSpinRewardsReturns::schema()),
            ("cats", "lobby_useSpin") => Some(CatsLobbyUseSpinReturns::schema()),
            ("cats", "lobby_completeTask") => Some(CatsLobbyCompleteTaskReturns::schema()),
            ("cats", "lobby_getCurrentTasks") => Some(CatsLobbyGetCurrentTasksReturns::schema()),
            ("cats", "lobby_getUsersCompletedTasks") => {
                Some(CatsLobbyGetUsersCompletedTasksReturns::schema())
            }
            ("cats", "lobby_userValidation") => Some(CatsLobbyUserValidationReturns::schema()),
            ("cats", "lobby_getBadTransactions") => {
                Some(CatsLobbyGetBadTransactionsReturns::schema())
            }
            ("cats", "lobby_getUserIdByTransactionId") => {
                Some(CatsLobbyGetUserIdByTransactionIdReturns::schema())
            }
            ("cats", "lobby_getUsersInappPurchases") => {
                Some(CatsLobbyGetUsersInappPurchasesReturns::schema())
            }
            ("cats", "lobby_resetDeviceId") => Some(CatsLobbyResetDeviceIdReturns::schema()),
            ("cats", "lobby_unlinkAccount") => Some(CatsLobbyUnlinkAccountReturns::schema()),
            ("cats", "lobby_getProfile") => Some(CatsLobbyGetProfileReturns::schema()),
            ("cats", "lobby_getSoulPrice") => Some(CatsLobbyGetSoulPriceReturns::schema()),
            ("cats", "lobby_purchaseValidation") => {
                Some(CatsLobbyPurchaseValidationReturns::schema())
            }
            ("cats", "lobby_retrieveUsersProgress") => {
                Some(CatsLobbyRetrieveUsersProgressReturns::schema())
            }
            ("cats", "lobby_shareScore") => Some(CatsLobbyShareScoreReturns::schema()),
            ("cats", "lobby_setUserAppMetricaDeviceId") => {
                Some(CatsLobbySetUserAppMetricaDeviceIdReturns::schema())
            }
            ("cats", "lobby_getJackpotWinnersInfo") => {
                Some(CatsLobbyGetJackpotWinnersInfoReturns::schema())
            }
            ("cats", "lobby_getProfileInfo") => Some(CatsLobbyGetProfileInfoReturns::schema()),
            ("cats", "lobby_applicationInitialization") => {
                Some(CatsLobbyApplicationInitializationReturns::schema())
            }
            ("catsAndDragons", "wrapper_getNicknameChangePrice") => {
                Some(CatsAndDragonsWrapperGetNicknameChangePriceReturns::schema())
            }
            ("catsAndDragons", "wrapper_nicknameChange") => {
                Some(CatsAndDragonsWrapperNicknameChangeReturns::schema())
            }
            ("catsAndDragons", "wrapper_collectNicknames") => {
                Some(CatsAndDragonsWrapperCollectNicknamesReturns::schema())
            }
            ("catsAndDragons", "wrapper_getUserIdByDeviceId") => {
                Some(CatsAndDragonsWrapperGetUserIdByDeviceIdReturns::schema())
            }
            ("catsAndDragons", "wrapper_getReferrals") => {
                Some(CatsAndDragonsWrapperGetReferralsReturns::schema())
            }
            ("catsAndDragons", "wrapper_getNickname") => {
                Some(CatsAndDragonsWrapperGetNicknameReturns::schema())
            }
            ("catsAndDragons", "wrapper_getUserInfo") => {
                Some(CatsAndDragonsWrapperGetUserInfoReturns::schema())
            }
            ("catsAndDragons", "wrapper_getMembershipsAndPricesAmount") => {
                Some(CatsAndDragonsWrapperGetMembershipsAndPricesAmountReturns::schema())
            }
            ("catsAndDragons", "wrapper_getUserMembershipInfo") => {
                Some(CatsAndDragonsWrapperGetUserMembershipInfoReturns::schema())
            }
            ("catsAndDragons", "wrapper_emptyMethod") => {
                Some(CatsAndDragonsWrapperEmptyMethodReturns::schema())
            }
            ("cd-balances", "balance_getBalancesByUserId") => {
                Some(CdBalancesBalanceGetBalancesByUserIdReturns::schema())
            }
            ("cd-balances", "balance_increaseBalanceByUserIdAndCurrency") => {
                Some(CdBalancesBalanceIncreaseBalanceByUserIdAndCurrencyReturns::schema())
            }
            ("cd-balances", "balance_decreaseBalanceByUserIdAndCurrency") => {
                Some(CdBalancesBalanceDecreaseBalanceByUserIdAndCurrencyReturns::schema())
            }
            ("cd-balances", "balance_getBalanceByUserIdAndCurrency") => {
                Some(CdBalancesBalanceGetBalanceByUserIdAndCurrencyReturns::schema())
            }
            ("cd-config", "config_addConfig") => Some(CdConfigConfigAddConfigReturns::schema()),
            ("cd-config", "config_getConfigByKey") => {
                Some(CdConfigConfigGetConfigByKeyReturns::schema())
            }
            ("cd-user", "spinEndpoints_spin") => Some(CdUserSpinEndpointsSpinReturns::schema()),
            ("cd-user", "spinEndpoints_getSpinInfo") => {
                Some(CdUserSpinEndpointsGetSpinInfoReturns::schema())
            }
            ("cron", "cron_create") => Some(CronCronCreateReturns::schema()),
            ("cron", "cron_update") => Some(CronCronUpdateReturns::schema()),
            ("cron", "cron_get") => Some(CronCronGetReturns::schema()),
            ("cron", "cron_delete") => Some(CronCronDeleteReturns::schema()),
            ("dragocats-balancer", "balancer_enterQueue") => {
                Some(DragocatsBalancerBalancerEnterQueueReturns::schema())
            }
            ("dragocats-balancer", "balancer_leaveQueue") => {
                Some(DragocatsBalancerBalancerLeaveQueueReturns::schema())
            }
            ("dragocats-balancer", "balancer_serverStarted") => {
                Some(DragocatsBalancerBalancerServerStartedReturns::schema())
            }
            ("dragocats-balancer", "balancer_serverStopped") => {
                Some(DragocatsBalancerBalancerServerStoppedReturns::schema())
            }
            ("dragocats-lobby", "stats_getAttributesByUnitId") => {
                Some(DragocatsLobbyStatsGetAttributesByUnitIdReturns::schema())
            }
            ("dragocats-lobby", "unitEndpoints_startBattle") => {
                Some(DragocatsLobbyUnitEndpointsStartBattleReturns::schema())
            }
            ("dragocats-lobby", "unitEndpoints_equipUnit") => {
                Some(DragocatsLobbyUnitEndpointsEquipUnitReturns::schema())
            }
            ("dragocats-lobby", "unitEndpoints_unEquipUnit") => {
                Some(DragocatsLobbyUnitEndpointsUnEquipUnitReturns::schema())
            }
            ("dragocats-lobby", "unitEndpoints_getEquipedUnits") => {
                Some(DragocatsLobbyUnitEndpointsGetEquipedUnitsReturns::schema())
            }
            ("dragocats-product-factory", "generator_generate") => {
                Some(DragocatsProductFactoryGeneratorGenerateReturns::schema())
            }
            ("dragocats-product-factory", "lootbox_open") => {
                Some(DragocatsProductFactoryLootboxOpenReturns::schema())
            }
            ("dragocats-product-factory", "products_getInfoByProductId") => {
                Some(DragocatsProductFactoryProductsGetInfoByProductIdReturns::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_getUserInventory") => {
                Some(DragocatsStorageInventoryEndpointsGetUserInventoryReturns::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_addLootboxNotification") => {
                Some(DragocatsStorageInventoryEndpointsAddLootboxNotificationReturns::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_addUnitNotification") => {
                Some(DragocatsStorageInventoryEndpointsAddUnitNotificationReturns::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_openLootbox") => {
                Some(DragocatsStorageInventoryEndpointsOpenLootboxReturns::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_openLootboxNotification") => {
                Some(DragocatsStorageInventoryEndpointsOpenLootboxNotificationReturns::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_equipUnit") => {
                Some(DragocatsStorageInventoryEndpointsEquipUnitReturns::schema())
            }
            ("dragocats-storage", "inventoryEndpoints_unEquipUnit") => {
                Some(DragocatsStorageInventoryEndpointsUnEquipUnitReturns::schema())
            }
            ("dragocats-storage", "unitEndpoints_getByUnitId") => {
                Some(DragocatsStorageUnitEndpointsGetByUnitIdReturns::schema())
            }
            ("dragons", "lobby_achievementComplete") => {
                Some(DragonsLobbyAchievementCompleteReturns::schema())
            }
            ("dragons", "lobby_getAllAchievements") => {
                Some(DragonsLobbyGetAllAchievementsReturns::schema())
            }
            ("dragons", "lobby_getUsersAchievements") => {
                Some(DragonsLobbyGetUsersAchievementsReturns::schema())
            }
            ("dragons", "lobby_adventureMapLevelUp") => {
                Some(DragonsLobbyAdventureMapLevelUpReturns::schema())
            }
            ("dragons", "lobby_getAdventureMapStats") => {
                Some(DragonsLobbyGetAdventureMapStatsReturns::schema())
            }
            ("dragons", "lobby_getApplicationSettings") => {
                Some(DragonsLobbyGetApplicationSettingsReturns::schema())
            }
            ("dragons", "lobby_craftBingoRecipe") => {
                Some(DragonsLobbyCraftBingoRecipeReturns::schema())
            }
            ("dragons", "lobby_getBingoData") => Some(DragonsLobbyGetBingoDataReturns::schema()),
            ("dragons", "lobby_getUsersCards") => Some(DragonsLobbyGetUsersCardsReturns::schema()),
            ("dragons", "lobby_upgradeCard") => Some(DragonsLobbyUpgradeCardReturns::schema()),
            ("dragons", "lobby_getClientVersion") => {
                Some(DragonsLobbyGetClientVersionReturns::schema())
            }
            ("dragons", "lobby_getConfigById") => Some(DragonsLobbyGetConfigByIdReturns::schema()),
            ("dragons", "lobby_updateConfig") => Some(DragonsLobbyUpdateConfigReturns::schema()),
            ("dragons", "lobby_buyCat") => Some(DragonsLobbyBuyCatReturns::schema()),
            ("dragons", "lobby_buyScience") => Some(DragonsLobbyBuyScienceReturns::schema()),
            ("dragons", "lobby_buySkill") => Some(DragonsLobbyBuySkillReturns::schema()),
            ("dragons", "lobby_buyUsualLootbox") => {
                Some(DragonsLobbyBuyUsualLootboxReturns::schema())
            }
            ("dragons", "lobby_getBingoJackpotPool") => {
                Some(DragonsLobbyGetBingoJackpotPoolReturns::schema())
            }
            ("dragons", "lobby_getBingoJackpotWinnersInfo") => {
                Some(DragonsLobbyGetBingoJackpotWinnersInfoReturns::schema())
            }
            ("dragons", "lobby_getLootboxJackpotWinnersInfo") => {
                Some(DragonsLobbyGetLootboxJackpotWinnersInfoReturns::schema())
            }
            ("dragons", "lobby_getLootboxJackpotPool") => {
                Some(DragonsLobbyGetLootboxJackpotPoolReturns::schema())
            }
            ("dragons", "lobby_getPreviousTournamentStats") => {
                Some(DragonsLobbyGetPreviousTournamentStatsReturns::schema())
            }
            ("dragons", "lobby_getSoftLeaderboardStats") => {
                Some(DragonsLobbyGetSoftLeaderboardStatsReturns::schema())
            }
            ("dragons", "lobby_getTournamentLeaderboardStats") => {
                Some(DragonsLobbyGetTournamentLeaderboardStatsReturns::schema())
            }
            ("dragons", "lobby_getLootboxesInfo") => {
                Some(DragonsLobbyGetLootboxesInfoReturns::schema())
            }
            ("dragons", "lobby_getUsualLootboxTime") => {
                Some(DragonsLobbyGetUsualLootboxTimeReturns::schema())
            }
            ("dragons", "lobby_openLootbox") => Some(DragonsLobbyOpenLootboxReturns::schema()),
            ("dragons", "lobby_getAvailableMembershipAmount") => {
                Some(DragonsLobbyGetAvailableMembershipAmountReturns::schema())
            }
            ("dragons", "lobby_getOffersAndBonuses") => {
                Some(DragonsLobbyGetOffersAndBonusesReturns::schema())
            }
            ("dragons", "lobby_refuseOffer") => Some(DragonsLobbyRefuseOfferReturns::schema()),
            ("dragons", "lobby_saveTutorialProgress") => {
                Some(DragonsLobbySaveTutorialProgressReturns::schema())
            }
            ("dragons", "lobby_saveUsersProgress") => {
                Some(DragonsLobbySaveUsersProgressReturns::schema())
            }
            ("dragons", "lobby_getUsersReferrals") => {
                Some(DragonsLobbyGetUsersReferralsReturns::schema())
            }
            ("dragons", "lobby_setReferrer") => Some(DragonsLobbySetReferrerReturns::schema()),
            ("dragons", "lobby_getServerTime") => Some(DragonsLobbyGetServerTimeReturns::schema()),
            ("dragons", "lobby_keepAlive") => Some(DragonsLobbyKeepAliveReturns::schema()),
            ("dragons", "lobby_getSpinRewards") => {
                Some(DragonsLobbyGetSpinRewardsReturns::schema())
            }
            ("dragons", "lobby_useSpin") => Some(DragonsLobbyUseSpinReturns::schema()),
            ("dragons", "lobby_completeTask") => Some(DragonsLobbyCompleteTaskReturns::schema()),
            ("dragons", "lobby_getCurrentTasks") => {
                Some(DragonsLobbyGetCurrentTasksReturns::schema())
            }
            ("dragons", "lobby_getUsersCompletedTasks") => {
                Some(DragonsLobbyGetUsersCompletedTasksReturns::schema())
            }
            ("dragons", "lobby_userValidation") => {
                Some(DragonsLobbyUserValidationReturns::schema())
            }
            ("dragons", "lobby_getBadTransactions") => {
                Some(DragonsLobbyGetBadTransactionsReturns::schema())
            }
            ("dragons", "lobby_getUserIdByTransactionId") => {
                Some(DragonsLobbyGetUserIdByTransactionIdReturns::schema())
            }
            ("dragons", "lobby_getUsersInappPurchases") => {
                Some(DragonsLobbyGetUsersInappPurchasesReturns::schema())
            }
            ("dragons", "lobby_resetDeviceId") => Some(DragonsLobbyResetDeviceIdReturns::schema()),
            ("dragons", "lobby_unlinkAccount") => Some(DragonsLobbyUnlinkAccountReturns::schema()),
            ("dragons", "lobby_getProfile") => Some(DragonsLobbyGetProfileReturns::schema()),
            ("dragons", "lobby_getSoulPrice") => Some(DragonsLobbyGetSoulPriceReturns::schema()),
            ("dragons", "lobby_purchaseValidation") => {
                Some(DragonsLobbyPurchaseValidationReturns::schema())
            }
            ("dragons", "lobby_retrieveUsersProgress") => {
                Some(DragonsLobbyRetrieveUsersProgressReturns::schema())
            }
            ("dragons", "lobby_shareScore") => Some(DragonsLobbyShareScoreReturns::schema()),
            ("dragons", "lobby_setUserAppMetricaDeviceId") => {
                Some(DragonsLobbySetUserAppMetricaDeviceIdReturns::schema())
            }
            ("dragons", "lobby_getJackpotWinnersInfo") => {
                Some(DragonsLobbyGetJackpotWinnersInfoReturns::schema())
            }
            ("dragons", "lobby_getProfileInfo") => {
                Some(DragonsLobbyGetProfileInfoReturns::schema())
            }
            ("dragons", "lobby_applicationInitialization") => {
                Some(DragonsLobbyApplicationInitializationReturns::schema())
            }
            ("email", "email_findAll") => Some(EmailEmailFindAllReturns::schema()),
            ("email", "email_createCronJob") => Some(EmailEmailCreateCronJobReturns::schema()),
            ("email", "email_createAndSend") => Some(EmailEmailCreateAndSendReturns::schema()),
            ("email", "email_sendNotSentLetters") => {
                Some(EmailEmailSendNotSentLettersReturns::schema())
            }
            ("email", "email_sendInProcessLetters") => {
                Some(EmailEmailSendInProcessLettersReturns::schema())
            }
            ("gameBalancer", "balancerEndpoints_addBots") => {
                Some(GameBalancerBalancerEndpointsAddBotsReturns::schema())
            }
            ("gameBalancer", "coefficientEndpoints_get") => {
                Some(GameBalancerCoefficientEndpointsGetReturns::schema())
            }
            ("gameBalancer", "coefficientEndpoints_update") => {
                Some(GameBalancerCoefficientEndpointsUpdateReturns::schema())
            }
            ("gameBalancer", "gameBalancer_addPlayerInSearch") => {
                Some(GameBalancerGameBalancerAddPlayerInSearchReturns::schema())
            }
            ("gameBalancer", "gameBalancer_disconnectPlayer") => {
                Some(GameBalancerGameBalancerDisconnectPlayerReturns::schema())
            }
            ("gameBalancer", "gameBalancer_deletePlayers") => {
                Some(GameBalancerGameBalancerDeletePlayersReturns::schema())
            }
            ("gameBalancer", "gameBalancer_stopSearching") => {
                Some(GameBalancerGameBalancerStopSearchingReturns::schema())
            }
            ("gameBalancer", "gameBalancer_gameStartedNotification") => {
                Some(GameBalancerGameBalancerGameStartedNotificationReturns::schema())
            }
            ("gameBalancer", "gameBalancer_checkInNotification") => {
                Some(GameBalancerGameBalancerCheckInNotificationReturns::schema())
            }
            ("google-play", "purchase_validateSubscription") => {
                Some(GooglePlayPurchaseValidateSubscriptionReturns::schema())
            }
            ("images", "image_uploadImage") => Some(ImagesImageUploadImageReturns::schema()),
            ("js-tests", "dragocatsBattleProcessEndpoints_startTest") => {
                Some(JsTestsDragocatsBattleProcessEndpointsStartTestReturns::schema())
            }
            ("js-tests", "dragocatsBattleProcessEndpoints_stopTest") => {
                Some(JsTestsDragocatsBattleProcessEndpointsStopTestReturns::schema())
            }
            ("js-tests", "dragocatsBattleProcessEndpoints_getTestState") => {
                Some(JsTestsDragocatsBattleProcessEndpointsGetTestStateReturns::schema())
            }
            ("listeria-storage", "heroesEndpoints_getById") => {
                Some(ListeriaStorageHeroesEndpointsGetByIdReturns::schema())
            }
            ("listeria-storage", "heroesEndpoints_levelUp") => {
                Some(ListeriaStorageHeroesEndpointsLevelUpReturns::schema())
            }
            ("listeria-storage", "heroesEndpoints_giveFreeExperienceToHero") => {
                Some(ListeriaStorageHeroesEndpointsGiveFreeExperienceToHeroReturns::schema())
            }
            ("listeria-storage", "heroesEndpoints_getHeroesListByUserId") => {
                Some(ListeriaStorageHeroesEndpointsGetHeroesListByUserIdReturns::schema())
            }
            ("listeria-storage", "heroesEndpoints_statsUpdatedNotification") => {
                Some(ListeriaStorageHeroesEndpointsStatsUpdatedNotificationReturns::schema())
            }
            ("listeria-storage", "heroesEndpoints_heroAddedNotification") => {
                Some(ListeriaStorageHeroesEndpointsHeroAddedNotificationReturns::schema())
            }
            ("listeria-storage", "inventoryEndpoints_getById") => {
                Some(ListeriaStorageInventoryEndpointsGetByIdReturns::schema())
            }
            ("listeria-storage", "inventoryEndpoints_getEquipmentScrollsCount") => {
                Some(ListeriaStorageInventoryEndpointsGetEquipmentScrollsCountReturns::schema())
            }
            ("listeria-storage", "inventoryEndpoints_getHeroScrollsCount") => {
                Some(ListeriaStorageInventoryEndpointsGetHeroScrollsCountReturns::schema())
            }
            ("listeria-storage", "inventoryEndpoints_getLootboxesList") => {
                Some(ListeriaStorageInventoryEndpointsGetLootboxesListReturns::schema())
            }
            ("listeria-storage", "inventoryEndpoints_openLootbox") => {
                Some(ListeriaStorageInventoryEndpointsOpenLootboxReturns::schema())
            }
            ("listeria-storage", "inventoryEndpoints_addedUnequippableItemNotification") => Some(
                ListeriaStorageInventoryEndpointsAddedUnequippableItemNotificationReturns::schema(),
            ),
            ("listeria-storage", "inventoryEndpoints_getItemsList") => {
                Some(ListeriaStorageInventoryEndpointsGetItemsListReturns::schema())
            }
            ("listeria-storage", "inventoryEndpoints_levelUp") => {
                Some(ListeriaStorageInventoryEndpointsLevelUpReturns::schema())
            }
            ("lobby", "equipmentEndpoints_equipItem") => {
                Some(LobbyEquipmentEndpointsEquipItemReturns::schema())
            }
            ("lobby", "equipmentEndpoints_unequipItem") => {
                Some(LobbyEquipmentEndpointsUnequipItemReturns::schema())
            }
            ("lobby", "equipmentEndpoints_getItemByItemIdAndUserId") => {
                Some(LobbyEquipmentEndpointsGetItemByItemIdAndUserIdReturns::schema())
            }
            ("lobby", "equipmentEndpoints_updatedItemNotification") => {
                Some(LobbyEquipmentEndpointsUpdatedItemNotificationReturns::schema())
            }
            ("lobby", "equipmentEndpoints_addedItemNotification") => {
                Some(LobbyEquipmentEndpointsAddedItemNotificationReturns::schema())
            }
            ("lobby", "equipmentEndpoints_unEquipItemFromAllHeroes") => {
                Some(LobbyEquipmentEndpointsUnEquipItemFromAllHeroesReturns::schema())
            }
            ("lobby", "heroesEndpoints_getHeroDTOAttributes") => {
                Some(LobbyHeroesEndpointsGetHeroDtoAttributesReturns::schema())
            }
            ("lobby", "heroesEndpoints_getEquippedItems") => {
                Some(LobbyHeroesEndpointsGetEquippedItemsReturns::schema())
            }
            ("lobby", "heroesEndpoints_equipHero") => {
                Some(LobbyHeroesEndpointsEquipHeroReturns::schema())
            }
            ("lobby", "heroesEndpoints_getHeroesList") => {
                Some(LobbyHeroesEndpointsGetHeroesListReturns::schema())
            }
            ("lobby", "heroesEndpoints_getActiveHeroId") => {
                Some(LobbyHeroesEndpointsGetActiveHeroIdReturns::schema())
            }
            ("lobby", "heroesEndpoints_heroAddedNotification") => {
                Some(LobbyHeroesEndpointsHeroAddedNotificationReturns::schema())
            }
            ("lobby", "lobby_startGame") => Some(LobbyLobbyStartGameReturns::schema()),
            ("lobby", "regions_get") => Some(LobbyRegionsGetReturns::schema()),
            ("lobby", "regions_set") => Some(LobbyRegionsSetReturns::schema()),
            ("lobby", "regions_getList") => Some(LobbyRegionsGetListReturns::schema()),
            ("lobby", "settings_get") => Some(LobbySettingsGetReturns::schema()),
            ("lobby", "settings_set") => Some(LobbySettingsSetReturns::schema()),
            ("lobby", "statsEndpoints_getLevelUpOptionsByPersonalType") => {
                Some(LobbyStatsEndpointsGetLevelUpOptionsByPersonalTypeReturns::schema())
            }
            ("lobby", "statsEndpoints_getStatsOptionsByPersonalType") => {
                Some(LobbyStatsEndpointsGetStatsOptionsByPersonalTypeReturns::schema())
            }
            ("lobby", "statsOptionsEndpoints_createOrUpdate") => {
                Some(LobbyStatsOptionsEndpointsCreateOrUpdateReturns::schema())
            }
            ("lobby", "statsOptionsEndpoints_getByPersonalType") => {
                Some(LobbyStatsOptionsEndpointsGetByPersonalTypeReturns::schema())
            }
            ("lobby", "statsOptionsEndpoints_getPersonalTypesList") => {
                Some(LobbyStatsOptionsEndpointsGetPersonalTypesListReturns::schema())
            }
            ("lobby", "user_getUserData") => Some(LobbyUserGetUserDataReturns::schema()),
            ("lobby", "user_updateUsername") => Some(LobbyUserUpdateUsernameReturns::schema()),
            ("lobby", "user_getLeaderBoard") => Some(LobbyUserGetLeaderBoardReturns::schema()),
            ("lobby", "user_updateProfileImage") => {
                Some(LobbyUserUpdateProfileImageReturns::schema())
            }
            ("lootboxes", "lootboxes_openLootbox") => {
                Some(LootboxesLootboxesOpenLootboxReturns::schema())
            }
            ("lootboxes", "lootboxes_getLootboxesInfo") => {
                Some(LootboxesLootboxesGetLootboxesInfoReturns::schema())
            }
            ("lootboxes", "lootboxes_getUsualLootboxTime") => {
                Some(LootboxesLootboxesGetUsualLootboxTimeReturns::schema())
            }
            ("market", "items_getPrice") => Some(MarketItemsGetPriceReturns::schema()),
            ("market", "items_getLootboxesList") => {
                Some(MarketItemsGetLootboxesListReturns::schema())
            }
            ("market-place", "marketEndpoints_getByFilter") => {
                Some(MarketPlaceMarketEndpointsGetByFilterReturns::schema())
            }
            ("market-place", "marketEndpoints_getSimilar") => {
                Some(MarketPlaceMarketEndpointsGetSimilarReturns::schema())
            }
            ("market-place", "marketPlace_buyItem") => {
                Some(MarketPlaceMarketPlaceBuyItemReturns::schema())
            }
            ("market-place", "marketPlace_unlockItem") => {
                Some(MarketPlaceMarketPlaceUnlockItemReturns::schema())
            }
            ("market-place", "marketPlace_getItemById") => {
                Some(MarketPlaceMarketPlaceGetItemByIdReturns::schema())
            }
            ("market-place", "marketPlace_getBoughtItems") => {
                Some(MarketPlaceMarketPlaceGetBoughtItemsReturns::schema())
            }
            ("market-place", "marketPlace_getItemsByFilter") => {
                Some(MarketPlaceMarketPlaceGetItemsByFilterReturns::schema())
            }
            ("market-place", "marketPlace_lockItem") => {
                Some(MarketPlaceMarketPlaceLockItemReturns::schema())
            }
            ("market-place", "marketPlace_cancelSale") => {
                Some(MarketPlaceMarketPlaceCancelSaleReturns::schema())
            }
            ("market-place", "marketPlace_getCanceledItems") => {
                Some(MarketPlaceMarketPlaceGetCanceledItemsReturns::schema())
            }
            ("market-place", "marketPlace_addToMarketPlace") => {
                Some(MarketPlaceMarketPlaceAddToMarketPlaceReturns::schema())
            }
            ("near-adapter", "contract_isEnoughBalanceOnWithdrawWallet") => {
                Some(NearAdapterContractIsEnoughBalanceOnWithdrawWalletReturns::schema())
            }
            ("near-adapter", "contract_callWithdraw") => {
                Some(NearAdapterContractCallWithdrawReturns::schema())
            }
            ("near-adapter", "contract_callTransfer") => {
                Some(NearAdapterContractCallTransferReturns::schema())
            }
            ("near-adapter", "contract_callMintNft") => {
                Some(NearAdapterContractCallMintNftReturns::schema())
            }
            ("near-adapter", "contract_callBurnNft") => {
                Some(NearAdapterContractCallBurnNftReturns::schema())
            }
            ("near-adapter", "contract_callNftLock") => {
                Some(NearAdapterContractCallNftLockReturns::schema())
            }
            ("near-adapter", "contract_callNftUnlock") => {
                Some(NearAdapterContractCallNftUnlockReturns::schema())
            }
            ("near-adapter", "contract_callNftUnlockAndTransfer") => {
                Some(NearAdapterContractCallNftUnlockAndTransferReturns::schema())
            }
            ("near-adapter", "wallet_getMy") => Some(NearAdapterWalletGetMyReturns::schema()),
            ("near-adapter", "wallet_getMyNft") => Some(NearAdapterWalletGetMyNftReturns::schema()),
            ("near-adapter", "wallet_getUserByAccountId") => {
                Some(NearAdapterWalletGetUserByAccountIdReturns::schema())
            }
            ("notifications", "notifications_createNew") => {
                Some(NotificationsNotificationsCreateNewReturns::schema())
            }
            ("orchestrator", "balance_increaseUserBalance") => {
                Some(OrchestratorBalanceIncreaseUserBalanceReturns::schema())
            }
            ("orchestrator", "balance_userBlockchainDeposit") => {
                Some(OrchestratorBalanceUserBlockchainDepositReturns::schema())
            }
            ("orchestrator", "balance_decreaseUserBalance") => {
                Some(OrchestratorBalanceDecreaseUserBalanceReturns::schema())
            }
            ("orchestrator", "battlePass_purchasePremium") => {
                Some(OrchestratorBattlePassPurchasePremiumReturns::schema())
            }
            ("orchestrator", "battlePass_purchaseExperience") => {
                Some(OrchestratorBattlePassPurchaseExperienceReturns::schema())
            }
            ("orchestrator", "battlePass_generateProduct") => {
                Some(OrchestratorBattlePassGenerateProductReturns::schema())
            }
            ("orchestrator", "bingo_reward") => Some(OrchestratorBingoRewardReturns::schema()),
            ("orchestrator", "lootbox_open") => Some(OrchestratorLootboxOpenReturns::schema()),
            ("orchestrator", "lootbox_purchase") => {
                Some(OrchestratorLootboxPurchaseReturns::schema())
            }
            ("orchestrator", "marketPlace_purchaseItem") => {
                Some(OrchestratorMarketPlacePurchaseItemReturns::schema())
            }
            ("orchestrator", "product_burn") => Some(OrchestratorProductBurnReturns::schema()),
            ("orchestrator", "purchase_purchaseProduct") => {
                Some(OrchestratorPurchasePurchaseProductReturns::schema())
            }
            ("orchestrator", "wallet_getMy") => Some(OrchestratorWalletGetMyReturns::schema()),
            ("orchestrator", "wallet_getByUserId") => {
                Some(OrchestratorWalletGetByUserIdReturns::schema())
            }
            ("product-factory", "generator_generate") => {
                Some(ProductFactoryGeneratorGenerateReturns::schema())
            }
            ("product-factory", "lootbox_open") => Some(ProductFactoryLootboxOpenReturns::schema()),
            ("product-factory", "products_getInfoByProductId") => {
                Some(ProductFactoryProductsGetInfoByProductIdReturns::schema())
            }
            ("productFactory", "lootboxType_add") => {
                Some(ProductFactoryLootboxTypeAddReturns::schema())
            }
            ("productFactory", "lootboxType_getAll") => {
                Some(ProductFactoryLootboxTypeGetAllReturns::schema())
            }
            ("productFactory", "lootboxType_updateByLootboxId") => {
                Some(ProductFactoryLootboxTypeUpdateByLootboxIdReturns::schema())
            }
            ("productFactory", "lootboxType_getByLootboxId") => {
                Some(ProductFactoryLootboxTypeGetByLootboxIdReturns::schema())
            }
            ("productFactory", "product_markHeroAsNFT") => {
                Some(ProductFactoryProductMarkHeroAsNftReturns::schema())
            }
            ("productFactory", "product_addLootbox") => {
                Some(ProductFactoryProductAddLootboxReturns::schema())
            }
            ("productFactory", "product_lootboxOpenedNotification") => {
                Some(ProductFactoryProductLootboxOpenedNotificationReturns::schema())
            }
            ("productFactory", "productType_add") => {
                Some(ProductFactoryProductTypeAddReturns::schema())
            }
            ("productFactory", "productType_isExist") => {
                Some(ProductFactoryProductTypeIsExistReturns::schema())
            }
            ("productFactory", "productType_update") => {
                Some(ProductFactoryProductTypeUpdateReturns::schema())
            }
            ("productFactory", "productType_delete") => {
                Some(ProductFactoryProductTypeDeleteReturns::schema())
            }
            ("productFactory", "productType_getAll") => {
                Some(ProductFactoryProductTypeGetAllReturns::schema())
            }
            ("productFactory", "productType_get") => {
                Some(ProductFactoryProductTypeGetReturns::schema())
            }
            ("productFactory", "productType_getAllByType") => {
                Some(ProductFactoryProductTypeGetAllByTypeReturns::schema())
            }
            ("productFactory", "productType_getAllByRarity") => {
                Some(ProductFactoryProductTypeGetAllByRarityReturns::schema())
            }
            ("productFactory", "productType_getHash") => {
                Some(ProductFactoryProductTypeGetHashReturns::schema())
            }
            ("productFactory", "productType_getAttributeListByPersonalType") => {
                Some(ProductFactoryProductTypeGetAttributeListByPersonalTypeReturns::schema())
            }
            ("promo", "codes_useCode") => Some(PromoCodesUseCodeReturns::schema()),
            ("promo", "codes_createCode") => Some(PromoCodesCreateCodeReturns::schema()),
            ("promo", "codes_getDataByCodeStrict") => {
                Some(PromoCodesGetDataByCodeStrictReturns::schema())
            }
            ("promo", "codes_getListStrict") => Some(PromoCodesGetListStrictReturns::schema()),
            ("promo", "codes_deleteCode") => Some(PromoCodesDeleteCodeReturns::schema()),
            ("purchase", "balance_increaseUserBalance") => {
                Some(PurchaseBalanceIncreaseUserBalanceReturns::schema())
            }
            ("purchase", "balance_decreaseUserBalance") => {
                Some(PurchaseBalanceDecreaseUserBalanceReturns::schema())
            }
            ("purchase", "balance_userBalanceChangedNotification") => {
                Some(PurchaseBalanceUserBalanceChangedNotificationReturns::schema())
            }
            ("purchase", "product_purchaseLootbox") => {
                Some(PurchaseProductPurchaseLootboxReturns::schema())
            }
            ("purchase", "product_getHeroList") => {
                Some(PurchaseProductGetHeroListReturns::schema())
            }
            ("purchase", "product_getEquipmentList") => {
                Some(PurchaseProductGetEquipmentListReturns::schema())
            }
            ("purchase", "wallet_getByUserId") => Some(PurchaseWalletGetByUserIdReturns::schema()),
            ("purchase", "wallet_getWalletByUserId") => {
                Some(PurchaseWalletGetWalletByUserIdReturns::schema())
            }
            ("realis", "walletManager_getMyAddress") => {
                Some(RealisWalletManagerGetMyAddressReturns::schema())
            }
            ("referral", "link_get") => Some(ReferralLinkGetReturns::schema()),
            ("referral", "link_getMy") => Some(ReferralLinkGetMyReturns::schema()),
            ("referral", "link_getAll") => Some(ReferralLinkGetAllReturns::schema()),
            ("referral", "link_getAllMy") => Some(ReferralLinkGetAllMyReturns::schema()),
            ("referral", "referral_getReferralInfoList") => {
                Some(ReferralReferralGetReferralInfoListReturns::schema())
            }
            ("referral", "referral_addReferral") => {
                Some(ReferralReferralAddReferralReturns::schema())
            }
            ("referral", "referral_setPotentialReferral") => {
                Some(ReferralReferralSetPotentialReferralReturns::schema())
            }
            ("referral", "referral_getReferrals") => {
                Some(ReferralReferralGetReferralsReturns::schema())
            }
            ("referral", "referral_getUserData") => {
                Some(ReferralReferralGetUserDataReturns::schema())
            }
            ("referral", "reward_addReferralExpense") => {
                Some(ReferralRewardAddReferralExpenseReturns::schema())
            }
            ("referral", "reward_makeRewardRequest") => {
                Some(ReferralRewardMakeRewardRequestReturns::schema())
            }
            ("referral", "reward_getAvailableReward") => {
                Some(ReferralRewardGetAvailableRewardReturns::schema())
            }
            ("refund", "balances_getAllMy") => Some(RefundBalancesGetAllMyReturns::schema()),
            ("refund", "balances_getAll") => Some(RefundBalancesGetAllReturns::schema()),
            ("refund", "balances_delete") => Some(RefundBalancesDeleteReturns::schema()),
            ("refund", "balances_getAllUnavailable") => {
                Some(RefundBalancesGetAllUnavailableReturns::schema())
            }
            ("refund", "balances_add") => Some(RefundBalancesAddReturns::schema()),
            ("refund", "balances_getAllLockedFunds") => {
                Some(RefundBalancesGetAllLockedFundsReturns::schema())
            }
            ("refund", "items_getAllMy") => Some(RefundItemsGetAllMyReturns::schema()),
            ("refund", "items_getAll") => Some(RefundItemsGetAllReturns::schema()),
            ("refund", "items_delete") => Some(RefundItemsDeleteReturns::schema()),
            ("refund", "items_isAvailable") => Some(RefundItemsIsAvailableReturns::schema()),
            ("refund", "items_getAllUnavailable") => {
                Some(RefundItemsGetAllUnavailableReturns::schema())
            }
            ("refund", "items_add") => Some(RefundItemsAddReturns::schema()),
            ("soul-adapter", "wallet_getUserIdByAddress") => {
                Some(SoulAdapterWalletGetUserIdByAddressReturns::schema())
            }
            ("soul-adapter", "wallet_getMyWallet") => {
                Some(SoulAdapterWalletGetMyWalletReturns::schema())
            }
            ("soul-adapter", "wallet_processTransactionByHash") => {
                Some(SoulAdapterWalletProcessTransactionByHashReturns::schema())
            }
            ("soul-adapter", "wallet_getBalanceByAddress") => {
                Some(SoulAdapterWalletGetBalanceByAddressReturns::schema())
            }
            ("status", "config_add") => Some(StatusConfigAddReturns::schema()),
            ("status", "config_delete") => Some(StatusConfigDeleteReturns::schema()),
            ("status", "config_disable") => Some(StatusConfigDisableReturns::schema()),
            ("status", "config_update") => Some(StatusConfigUpdateReturns::schema()),
            ("status", "config_getAll") => Some(StatusConfigGetAllReturns::schema()),
            ("status", "config_getMembershipInfo") => {
                Some(StatusConfigGetMembershipInfoReturns::schema())
            }
            ("status", "config_updatePriorityIndex") => {
                Some(StatusConfigUpdatePriorityIndexReturns::schema())
            }
            ("status", "config_getList") => Some(StatusConfigGetListReturns::schema()),
            ("status", "config_getAllForPurchase") => {
                Some(StatusConfigGetAllForPurchaseReturns::schema())
            }
            ("status", "membership_update") => Some(StatusMembershipUpdateReturns::schema()),
            ("status", "membership_addAwardMembership") => {
                Some(StatusMembershipAddAwardMembershipReturns::schema())
            }
            ("status", "membership_getInfo") => Some(StatusMembershipGetInfoReturns::schema()),
            ("status", "membership_changeMembershipGame") => {
                Some(StatusMembershipChangeMembershipGameReturns::schema())
            }
            ("status", "membership_disable") => Some(StatusMembershipDisableReturns::schema()),
            ("status", "membership_getAllActive") => {
                Some(StatusMembershipGetAllActiveReturns::schema())
            }
            ("status", "membership_getAllMyActive") => {
                Some(StatusMembershipGetAllMyActiveReturns::schema())
            }
            ("status", "membership_purchaseMembershipFromSite") => {
                Some(StatusMembershipPurchaseMembershipFromSiteReturns::schema())
            }
            ("status", "membership_getUserByGPA") => {
                Some(StatusMembershipGetUserByGpaReturns::schema())
            }
            ("status", "membership_checkUserHasActiveStatus") => {
                Some(StatusMembershipCheckUserHasActiveStatusReturns::schema())
            }
            ("task", "task_completeTask") => Some(TaskTaskCompleteTaskReturns::schema()),
            ("task", "task_getCurrentTasks") => Some(TaskTaskGetCurrentTasksReturns::schema()),
            ("task", "task_getUsersCompletedTasks") => {
                Some(TaskTaskGetUsersCompletedTasksReturns::schema())
            }
            ("transactions", "balance_getBalanceByUserId") => {
                Some(TransactionsBalanceGetBalanceByUserIdReturns::schema())
            }
            ("transactions", "balance_getBalancesByUserId") => {
                Some(TransactionsBalanceGetBalancesByUserIdReturns::schema())
            }
            ("transactions", "balance_getBalancesByUserIdAsArray") => {
                Some(TransactionsBalanceGetBalancesByUserIdAsArrayReturns::schema())
            }
            ("transactions", "balance_increaseBalanceByUserId") => {
                Some(TransactionsBalanceIncreaseBalanceByUserIdReturns::schema())
            }
            ("transactions", "balance_decreaseBalanceByUserId") => {
                Some(TransactionsBalanceDecreaseBalanceByUserIdReturns::schema())
            }
            ("transactions", "balance_getBalancesInUsd") => {
                Some(TransactionsBalanceGetBalancesInUsdReturns::schema())
            }
            ("transactions", "balance_convert") => {
                Some(TransactionsBalanceConvertReturns::schema())
            }
            ("transactions", "balance_getMyBalancesWithRounding") => {
                Some(TransactionsBalanceGetMyBalancesWithRoundingReturns::schema())
            }
            ("transactions", "balance_getAll") => Some(TransactionsBalanceGetAllReturns::schema()),
            ("transactions", "balance_getAllMy") => {
                Some(TransactionsBalanceGetAllMyReturns::schema())
            }
            ("transactions", "balance_getMyNumOfTransactions") => {
                Some(TransactionsBalanceGetMyNumOfTransactionsReturns::schema())
            }
            ("transactions", "balance_getAllCreditTransactionList") => {
                Some(TransactionsBalanceGetAllCreditTransactionListReturns::schema())
            }
            ("transactions", "balance_getUserBalances") => {
                Some(TransactionsBalanceGetUserBalancesReturns::schema())
            }
            ("transactions", "balance_getWithFilter") => {
                Some(TransactionsBalanceGetWithFilterReturns::schema())
            }
            ("transactions", "balance_getNumOfTransactions") => {
                Some(TransactionsBalanceGetNumOfTransactionsReturns::schema())
            }
            ("transactions", "balance_updateTransactionHashAndBlockId") => {
                Some(TransactionsBalanceUpdateTransactionHashAndBlockIdReturns::schema())
            }
            ("transactions", "balance_updateTransactionDataFromBlockchain") => {
                Some(TransactionsBalanceUpdateTransactionDataFromBlockchainReturns::schema())
            }
            ("transactions", "balance_getListWithPagination") => {
                Some(TransactionsBalanceGetListWithPaginationReturns::schema())
            }
            ("transactions", "balance_deleteBalanceByUserId") => {
                Some(TransactionsBalanceDeleteBalanceByUserIdReturns::schema())
            }
            ("transactions", "balance_getAllLisSum") => {
                Some(TransactionsBalanceGetAllLisSumReturns::schema())
            }
            ("transactions", "balance_adminIncreaseBalance") => {
                Some(TransactionsBalanceAdminIncreaseBalanceReturns::schema())
            }
            ("transactions", "balance_adminDecreaseBalance") => {
                Some(TransactionsBalanceAdminDecreaseBalanceReturns::schema())
            }
            ("transactions", "balance_getTestnetLis") => {
                Some(TransactionsBalanceGetTestnetLisReturns::schema())
            }
            ("transactions", "balance_getNumWithFilter") => {
                Some(TransactionsBalanceGetNumWithFilterReturns::schema())
            }
            ("transactions", "balance_checkTransactionForCurrency") => {
                Some(TransactionsBalanceCheckTransactionForCurrencyReturns::schema())
            }
            ("transactions", "balance_checkMyTransactionForCurrency") => {
                Some(TransactionsBalanceCheckMyTransactionForCurrencyReturns::schema())
            }
            ("transactions", "registryProduct_addProduct") => {
                Some(TransactionsRegistryProductAddProductReturns::schema())
            }
            ("transactions", "registryProduct_addProductHash") => {
                Some(TransactionsRegistryProductAddProductHashReturns::schema())
            }
            ("transactions", "registryProduct_getPersonalTypesByUser") => {
                Some(TransactionsRegistryProductGetPersonalTypesByUserReturns::schema())
            }
            ("transactions", "registryProduct_deleteProductByUserId") => {
                Some(TransactionsRegistryProductDeleteProductByUserIdReturns::schema())
            }
            ("transactions", "registryProduct_burnProduct") => {
                Some(TransactionsRegistryProductBurnProductReturns::schema())
            }
            ("transactions", "registryProduct_updateProductOwner") => {
                Some(TransactionsRegistryProductUpdateProductOwnerReturns::schema())
            }
            ("user", "profile_changeNickname") => Some(UserProfileChangeNicknameReturns::schema()),
            ("user", "profile_getProfile") => Some(UserProfileGetProfileReturns::schema()),
            ("user", "profile_getMyProfile") => Some(UserProfileGetMyProfileReturns::schema()),
            ("user", "profile_getAllProfiles") => Some(UserProfileGetAllProfilesReturns::schema()),
            ("user", "profile_unsetNotice") => Some(UserProfileUnsetNoticeReturns::schema()),
            ("user", "profile_setSuspicious") => Some(UserProfileSetSuspiciousReturns::schema()),
            ("user", "profile_unsetSuspicious") => {
                Some(UserProfileUnsetSuspiciousReturns::schema())
            }
            ("user", "profile_banProfile") => Some(UserProfileBanProfileReturns::schema()),
            ("user", "profile_unBan") => Some(UserProfileUnBanReturns::schema()),
            ("user", "profile_setMailingSubscriptionStatus") => {
                Some(UserProfileSetMailingSubscriptionStatusReturns::schema())
            }
            ("user", "profile_unsubscribeFromNewsletter") => {
                Some(UserProfileUnsubscribeFromNewsletterReturns::schema())
            }
            ("user", "profile_getAllUsersSubscribedToMailing") => {
                Some(UserProfileGetAllUsersSubscribedToMailingReturns::schema())
            }
            ("user", "profile_getBanInfo") => Some(UserProfileGetBanInfoReturns::schema()),
            ("user", "profile_getMyBanInfo") => Some(UserProfileGetMyBanInfoReturns::schema()),
            ("user", "profile_getNicknameChangePrice") => {
                Some(UserProfileGetNicknameChangePriceReturns::schema())
            }
            ("user", "profile_getUserByParams") => {
                Some(UserProfileGetUserByParamsReturns::schema())
            }
            ("user", "profile_getMyNickname") => Some(UserProfileGetMyNicknameReturns::schema()),
            ("user", "profile_getUserNickname") => {
                Some(UserProfileGetUserNicknameReturns::schema())
            }
            ("user", "profile_getNicknamesByUserIds") => {
                Some(UserProfileGetNicknamesByUserIdsReturns::schema())
            }
            ("user", "profile_deleteUser") => Some(UserProfileDeleteUserReturns::schema()),
            ("user", "profile_setNotice") => Some(UserProfileSetNoticeReturns::schema()),
            ("user", "profile_changeEmail") => Some(UserProfileChangeEmailReturns::schema()),
            ("user", "profile_isEmailExists") => Some(UserProfileIsEmailExistsReturns::schema()),
            ("user", "profile_getUserIdByEmail") => {
                Some(UserProfileGetUserIdByEmailReturns::schema())
            }
            ("user", "profile_deleteUserRecord") => {
                Some(UserProfileDeleteUserRecordReturns::schema())
            }
            ("user", "profile_getNum") => Some(UserProfileGetNumReturns::schema()),
            ("user", "profile_getMyProfileForBytes") => {
                Some(UserProfileGetMyProfileForBytesReturns::schema())
            }
            ("user", "profile_getTestData") => Some(UserProfileGetTestDataReturns::schema()),
            ("user", "profile_getCountActives") => {
                Some(UserProfileGetCountActivesReturns::schema())
            }
            ("user", "status_delete") => Some(UserStatusDeleteReturns::schema()),
            ("user", "status_get") => Some(UserStatusGetReturns::schema()),
            ("withdraw", "approval_getAllMyTransactions") => {
                Some(WithdrawApprovalGetAllMyTransactionsReturns::schema())
            }
            ("withdraw", "approval_listUnapproved") => {
                Some(WithdrawApprovalListUnapprovedReturns::schema())
            }
            ("withdraw", "approval_approve") => Some(WithdrawApprovalApproveReturns::schema()),
            ("withdraw", "approval_deny") => Some(WithdrawApprovalDenyReturns::schema()),
            ("withdraw", "attempt_tryNew") => Some(WithdrawAttemptTryNewReturns::schema()),
            _ => None,
        }
    }
    pub fn get_access_level(agent: &str, method: &str) -> Option<AccessLevel> {
        match (agent, method) {
            ("achievements", "achievement_achievementComplete") => AccessLevel::Protected,
            ("achievements", "achievement_getAllAchievements") => AccessLevel::Public,
            ("achievements", "achievement_getUsersAchievements") => AccessLevel::Public,
            ("admin", "action_undo") => AccessLevel::Private,
            ("admin", "action_getActionList") => AccessLevel::Private,
            ("admin", "action_deleteByActionId") => AccessLevel::Private,
            ("admin", "action_getAllByFilterList") => AccessLevel::Private,
            ("admin", "confirmation_addSolutionAdmin") => AccessLevel::Private,
            ("admin", "confirmation_deleteAction") => AccessLevel::Private,
            ("admin", "confirmation_getAllActions") => AccessLevel::Private,
            ("admin", "confirmation_getNotConfirmedActions") => AccessLevel::Private,
            ("admin", "confirmation_add") => AccessLevel::Internal,
            ("admin", "confirmation_addSolutionAdminForMany") => AccessLevel::Private,
            ("admin", "mailTemplate_create") => AccessLevel::Private,
            ("admin", "mailTemplate_delete") => AccessLevel::Private,
            ("admin", "mailTemplate_getByKey") => AccessLevel::Private,
            ("admin", "mailTemplate_change") => AccessLevel::Private,
            ("admin", "mailTemplate_getAll") => AccessLevel::Private,
            ("admin", "option_set") => AccessLevel::Private,
            ("admin", "option_unset") => AccessLevel::Private,
            ("admin", "option_get") => AccessLevel::Protected,
            ("admin", "option_getList") => AccessLevel::Private,
            ("admin", "option_getByScope") => AccessLevel::Private,
            ("admin", "option_getAllByFilter") => AccessLevel::Private,
            ("admin", "option_update") => AccessLevel::Private,
            ("admin", "option_updateAll") => AccessLevel::Private,
            ("admin", "permission_add") => AccessLevel::Private,
            ("admin", "permission_delete") => AccessLevel::Private,
            ("admin", "permission_updatePermissions") => AccessLevel::Private,
            ("admin", "role_add") => AccessLevel::Private,
            ("admin", "role_delete") => AccessLevel::Private,
            ("admin", "role_getUsersByRole") => AccessLevel::Private,
            ("admin", "role_getPermissionsByRoleName") => AccessLevel::Private,
            ("admin", "role_getAllRoles") => AccessLevel::Private,
            ("admin", "role_getRoleByName") => AccessLevel::Private,
            ("admin", "userRole_add") => AccessLevel::Private,
            ("admin", "userRole_getAll") => AccessLevel::Private,
            ("admin", "userRole_deleteUserRole") => AccessLevel::Private,
            ("admin", "userRole_getAllWithNickname") => AccessLevel::Private,
            ("admin", "userRole_getMyRole") => AccessLevel::Protected,
            ("admin", "userRole_getAllUsersWithNickname") => AccessLevel::Private,
            ("admin", "userRole_isAdmin") => AccessLevel::Protected,
            ("analytics", "analytics_send") => AccessLevel::Public,
            ("auth", "admin_login") => AccessLevel::Public,
            ("auth", "admin_addUserRole") => AccessLevel::Private,
            ("auth", "admin_deleteUserRole") => AccessLevel::Private,
            ("auth", "admin_updateUserRole") => AccessLevel::Private,
            ("auth", "admin_getUserRole") => AccessLevel::Private,
            ("auth", "admin_getMyRole") => AccessLevel::Protected,
            ("auth", "admin_getUsers") => AccessLevel::Private,
            ("auth", "auth_login") => AccessLevel::Public,
            ("auth", "auth_socialNetworkAuth") => AccessLevel::Public,
            ("auth", "auth_changePassword") => AccessLevel::Public,
            ("auth", "auth_sendRequestToResetPassword") => AccessLevel::Public,
            ("auth", "auth_resetPassword") => AccessLevel::Public,
            ("auth", "auth_sendRequestToDeleteAccount") => AccessLevel::Protected,
            ("auth", "auth_removeAccount") => AccessLevel::Public,
            ("auth", "auth_getAccessTokenByRefresh") => AccessLevel::Public,
            ("auth", "auth_getAllMyActiveSessions") => AccessLevel::Protected,
            ("auth", "auth_confirmPassword") => AccessLevel::Protected,
            ("auth", "auth_deleteMyUser") => AccessLevel::Protected,
            ("auth", "auth_getUserInfoByToken") => AccessLevel::Protected,
            ("auth", "auth_createRequestToConfirmEmail") => AccessLevel::Public,
            ("auth", "auth_confirmEmailByCode") => AccessLevel::Public,
            ("auth", "auth_confirmEmailByHash") => AccessLevel::Public,
            ("auth", "auth_deleteEmailRequest") => AccessLevel::Public,
            ("auth", "auth_setPassword") => AccessLevel::Public,
            ("auth", "auth_disableUser") => AccessLevel::Internal,
            ("auth", "auth_resentConfirmationMail") => AccessLevel::Public,
            ("auth", "auth_createRequestToChangeEmail") => AccessLevel::Internal,
            ("auth", "auth_changeMyEmail") => AccessLevel::Internal,
            ("auth", "auth_loginWithFacebook") => AccessLevel::Protected,
            ("auth", "auth_logout") => AccessLevel::Protected,
            ("auth", "auth_emptyMethod") => AccessLevel::Public,
            ("auth", "auth_assignProviderAccountToDeviceId") => AccessLevel::Protected,
            ("auth", "auth_resetUserProgress") => AccessLevel::Private,
            ("auth", "authDevice_getOrCreateInternalUserId") => AccessLevel::Internal,
            ("auth", "authDevice_isNewClient") => AccessLevel::Internal,
            ("auth", "authDevice_isAuthorized") => AccessLevel::Internal,
            ("auth", "authDevice_getClientInfo") => AccessLevel::Internal,
            ("auth", "authDevice_updateInternalIdByDeviceId") => AccessLevel::Internal,
            ("auth", "authDevice_getClientStatusByInternalUserId") => AccessLevel::Internal,
            ("auth", "ban_ban") => AccessLevel::Private,
            ("auth", "ban_unBan") => AccessLevel::Private,
            ("auth", "ban_getUserBan") => AccessLevel::Private,
            ("auth", "instantMigration_initMigration") => AccessLevel::Private,
            ("auth", "instantMigration_stopMigration") => AccessLevel::Private,
            ("auth", "role_add") => AccessLevel::Private,
            ("auth", "role_delete") => AccessLevel::Private,
            ("auth", "role_update") => AccessLevel::Private,
            ("auth", "role_getAllRoles") => AccessLevel::Private,
            ("auth", "role_getRoleByName") => AccessLevel::Private,
            ("auth", "twoFactor_getSecret") => AccessLevel::Protected,
            ("auth", "twoFactor_verifyCode") => AccessLevel::Protected,
            ("auth", "twoFactor_disableTwoFA") => AccessLevel::Protected,
            ("auth", "twoFactor_enableTwoFA") => AccessLevel::Protected,
            ("auth", "twoFactor_isRequiredTwoFA") => AccessLevel::Protected,
            ("auth", "twoFactor_getStatus") => AccessLevel::Protected,
            ("auth", "twoFactor_sendRequestToDeleteTwoFA") => AccessLevel::Protected,
            ("auth", "twoFactor_deleteTwoFA") => AccessLevel::Public,
            ("balances", "balances_freeExperienceAddedNotification") => AccessLevel::Protected,
            ("balances", "balances_getSoftCurrencyBalance") => AccessLevel::Protected,
            ("balances", "balances_increaseSoftCurrency") => AccessLevel::Internal,
            ("balances", "balances_decreaseSoftCurrency") => AccessLevel::Internal,
            ("balances", "balances_addFreeExperience") => AccessLevel::Internal,
            ("balances", "balances_useFreeExperience") => AccessLevel::Internal,
            ("balances", "balances_getBalancesByUserId") => AccessLevel::Protected,
            ("balances", "balances_getFreeExperienceBalance") => AccessLevel::Protected,
            ("battle-pass", "battlePassEndpoints_receiveReward") => AccessLevel::Protected,
            ("battle-pass", "battlePassEndpoints_getBattlePassData") => AccessLevel::Protected,
            ("battle-pass", "battlePassEndpoints_getCurrentBattlePassExperience") => {
                AccessLevel::Protected
            }
            ("battle-pass", "battlePassEndpoints_getBattlePassSeasonInfo") => {
                AccessLevel::Protected
            }
            ("battle-pass", "battlePassEndpoints_getBattlePassPrices") => AccessLevel::Protected,
            ("battle-pass", "battlePassEndpoints_receiveOldBattlePassRewards") => {
                AccessLevel::Protected
            }
            ("battle-pass", "battlePassEndpoints_purchaseBattlePassExperienceToLevelup") => {
                AccessLevel::Protected
            }
            ("battle-pass", "battlePassEndpoints_purchaseBattlePassPremium") => {
                AccessLevel::Protected
            }
            ("binance-wallet", "wallet_createWallet") => AccessLevel::Protected,
            ("binance-wallet", "wallet_getAddressByUserId") => AccessLevel::Internal,
            ("binance-wallet", "wallet_getUserIdByAddress") => AccessLevel::Internal,
            ("bingo", "bingo_getBingoData") => AccessLevel::Internal,
            ("bingo", "bingo_craftBingoRecipe") => AccessLevel::Internal,
            ("bingo", "bingo_getBingoJackpotPool") => AccessLevel::Internal,
            ("bingo", "bingo_getBingoJackpotWinnersInfo") => AccessLevel::Internal,
            ("bingo", "bingo_getCurrentBingoSeason") => AccessLevel::Internal,
            ("blog", "blog_create") => AccessLevel::Private,
            ("blog", "blog_update") => AccessLevel::Private,
            ("blog", "blog_delete") => AccessLevel::Private,
            ("blog", "blog_getAll") => AccessLevel::Private,
            ("blog", "blog_get") => AccessLevel::Private,
            ("blog", "blog_getByUrl") => AccessLevel::Public,
            ("blog", "blog_getPinned") => AccessLevel::Public,
            ("blog", "blog_getAllByFilter") => AccessLevel::Public,
            ("blog", "blog_getAllWithCategoryList") => AccessLevel::Public,
            ("blog", "category_create") => AccessLevel::Private,
            ("blog", "category_update") => AccessLevel::Private,
            ("blog", "category_delete") => AccessLevel::Private,
            ("blog", "category_getAll") => AccessLevel::Public,
            ("blog", "category_get") => AccessLevel::Private,
            ("blog", "poll_add") => AccessLevel::Private,
            ("blog", "poll_get") => AccessLevel::Public,
            ("blog", "poll_getAll") => AccessLevel::Private,
            ("blog", "poll_update") => AccessLevel::Private,
            ("blog", "poll_delete") => AccessLevel::Private,
            ("blog", "vote_addVote") => AccessLevel::Protected,
            ("blog", "vote_getAllVotesByPollId") => AccessLevel::Protected,
            ("blog", "vote_isVoted") => AccessLevel::Protected,
            ("cats", "lobby_achievementComplete") => AccessLevel::Protected,
            ("cats", "lobby_getAllAchievements") => AccessLevel::Public,
            ("cats", "lobby_getUsersAchievements") => AccessLevel::Public,
            ("cats", "lobby_adventureMapLevelUp") => AccessLevel::Public,
            ("cats", "lobby_getAdventureMapStats") => AccessLevel::Public,
            ("cats", "lobby_getApplicationSettings") => AccessLevel::Public,
            ("cats", "lobby_craftBingoRecipe") => AccessLevel::Protected,
            ("cats", "lobby_getBingoData") => AccessLevel::Public,
            ("cats", "lobby_getUsersCards") => AccessLevel::Public,
            ("cats", "lobby_upgradeCard") => AccessLevel::Protected,
            ("cats", "lobby_getClientVersion") => AccessLevel::Public,
            ("cats", "lobby_getConfigById") => AccessLevel::Public,
            ("cats", "lobby_updateConfig") => AccessLevel::Public,
            ("cats", "lobby_buyCat") => AccessLevel::Protected,
            ("cats", "lobby_buyScience") => AccessLevel::Protected,
            ("cats", "lobby_buySkill") => AccessLevel::Protected,
            ("cats", "lobby_buyUsualLootbox") => AccessLevel::Protected,
            ("cats", "lobby_getBingoJackpotPool") => AccessLevel::Public,
            ("cats", "lobby_getBingoJackpotWinnersInfo") => AccessLevel::Public,
            ("cats", "lobby_getLootboxJackpotWinnersInfo") => AccessLevel::Public,
            ("cats", "lobby_getLootboxJackpotPool") => AccessLevel::Public,
            ("cats", "lobby_getPreviousTournamentStats") => AccessLevel::Public,
            ("cats", "lobby_getSoftLeaderboardStats") => AccessLevel::Public,
            ("cats", "lobby_getTournamentLeaderboardStats") => AccessLevel::Public,
            ("cats", "lobby_getLootboxesInfo") => AccessLevel::Public,
            ("cats", "lobby_getUsualLootboxTime") => AccessLevel::Public,
            ("cats", "lobby_openLootbox") => AccessLevel::Protected,
            ("cats", "lobby_getAvailableMembershipAmount") => AccessLevel::Public,
            ("cats", "lobby_getOffersAndBonuses") => AccessLevel::Public,
            ("cats", "lobby_refuseOffer") => AccessLevel::Public,
            ("cats", "lobby_saveTutorialProgress") => AccessLevel::Public,
            ("cats", "lobby_saveUsersProgress") => AccessLevel::Public,
            ("cats", "lobby_getUsersReferrals") => AccessLevel::Protected,
            ("cats", "lobby_setReferrer") => AccessLevel::Public,
            ("cats", "lobby_getServerTime") => AccessLevel::Public,
            ("cats", "lobby_keepAlive") => AccessLevel::Public,
            ("cats", "lobby_getSpinRewards") => AccessLevel::Public,
            ("cats", "lobby_useSpin") => AccessLevel::Public,
            ("cats", "lobby_completeTask") => AccessLevel::Protected,
            ("cats", "lobby_getCurrentTasks") => AccessLevel::Public,
            ("cats", "lobby_getUsersCompletedTasks") => AccessLevel::Public,
            ("cats", "lobby_userValidation") => AccessLevel::Public,
            ("cats", "lobby_getBadTransactions") => AccessLevel::Public,
            ("cats", "lobby_getUserIdByTransactionId") => AccessLevel::Public,
            ("cats", "lobby_getUsersInappPurchases") => AccessLevel::Public,
            ("cats", "lobby_resetDeviceId") => AccessLevel::Protected,
            ("cats", "lobby_unlinkAccount") => AccessLevel::Protected,
            ("cats", "lobby_getProfile") => AccessLevel::Public,
            ("cats", "lobby_getSoulPrice") => AccessLevel::Public,
            ("cats", "lobby_purchaseValidation") => AccessLevel::Public,
            ("cats", "lobby_retrieveUsersProgress") => AccessLevel::Public,
            ("cats", "lobby_shareScore") => AccessLevel::Public,
            ("cats", "lobby_setUserAppMetricaDeviceId") => AccessLevel::Public,
            ("cats", "lobby_getJackpotWinnersInfo") => AccessLevel::Public,
            ("cats", "lobby_getProfileInfo") => AccessLevel::Public,
            ("cats", "lobby_applicationInitialization") => AccessLevel::Public,
            ("catsAndDragons", "wrapper_getNicknameChangePrice") => AccessLevel::Public,
            ("catsAndDragons", "wrapper_nicknameChange") => AccessLevel::Public,
            ("catsAndDragons", "wrapper_collectNicknames") => AccessLevel::Public,
            ("catsAndDragons", "wrapper_getUserIdByDeviceId") => AccessLevel::Public,
            ("catsAndDragons", "wrapper_getReferrals") => AccessLevel::Public,
            ("catsAndDragons", "wrapper_getNickname") => AccessLevel::Public,
            ("catsAndDragons", "wrapper_getUserInfo") => AccessLevel::Public,
            ("catsAndDragons", "wrapper_getMembershipsAndPricesAmount") => AccessLevel::Public,
            ("catsAndDragons", "wrapper_getUserMembershipInfo") => AccessLevel::Public,
            ("catsAndDragons", "wrapper_emptyMethod") => AccessLevel::Public,
            ("cd-balances", "balance_getBalancesByUserId") => AccessLevel::Protected,
            ("cd-balances", "balance_increaseBalanceByUserIdAndCurrency") => AccessLevel::Internal,
            ("cd-balances", "balance_decreaseBalanceByUserIdAndCurrency") => AccessLevel::Internal,
            ("cd-balances", "balance_getBalanceByUserIdAndCurrency") => AccessLevel::Protected,
            ("cd-config", "config_addConfig") => AccessLevel::Public,
            ("cd-config", "config_getConfigByKey") => AccessLevel::Public,
            ("cd-user", "spinEndpoints_spin") => AccessLevel::Protected,
            ("cd-user", "spinEndpoints_getSpinInfo") => AccessLevel::Protected,
            ("cron", "cron_create") => AccessLevel::Internal,
            ("cron", "cron_update") => AccessLevel::Internal,
            ("cron", "cron_get") => AccessLevel::Internal,
            ("cron", "cron_delete") => AccessLevel::Internal,
            ("dragocats-balancer", "balancer_enterQueue") => AccessLevel::Internal,
            ("dragocats-balancer", "balancer_leaveQueue") => AccessLevel::Internal,
            ("dragocats-balancer", "balancer_serverStarted") => AccessLevel::Internal,
            ("dragocats-balancer", "balancer_serverStopped") => AccessLevel::Internal,
            ("dragocats-lobby", "stats_getAttributesByUnitId") => AccessLevel::Internal,
            ("dragocats-lobby", "unitEndpoints_startBattle") => AccessLevel::Protected,
            ("dragocats-lobby", "unitEndpoints_equipUnit") => AccessLevel::Protected,
            ("dragocats-lobby", "unitEndpoints_unEquipUnit") => AccessLevel::Protected,
            ("dragocats-lobby", "unitEndpoints_getEquipedUnits") => AccessLevel::Protected,
            ("dragocats-product-factory", "generator_generate") => AccessLevel::Protected,
            ("dragocats-product-factory", "lootbox_open") => AccessLevel::Protected,
            ("dragocats-product-factory", "products_getInfoByProductId") => AccessLevel::Internal,
            ("dragocats-storage", "inventoryEndpoints_getUserInventory") => AccessLevel::Protected,
            ("dragocats-storage", "inventoryEndpoints_addLootboxNotification") => {
                AccessLevel::Protected
            }
            ("dragocats-storage", "inventoryEndpoints_addUnitNotification") => {
                AccessLevel::Protected
            }
            ("dragocats-storage", "inventoryEndpoints_openLootbox") => AccessLevel::Protected,
            ("dragocats-storage", "inventoryEndpoints_openLootboxNotification") => {
                AccessLevel::Protected
            }
            ("dragocats-storage", "inventoryEndpoints_equipUnit") => AccessLevel::Protected,
            ("dragocats-storage", "inventoryEndpoints_unEquipUnit") => AccessLevel::Protected,
            ("dragocats-storage", "unitEndpoints_getByUnitId") => AccessLevel::Protected,
            ("dragons", "lobby_achievementComplete") => AccessLevel::Protected,
            ("dragons", "lobby_getAllAchievements") => AccessLevel::Public,
            ("dragons", "lobby_getUsersAchievements") => AccessLevel::Public,
            ("dragons", "lobby_adventureMapLevelUp") => AccessLevel::Public,
            ("dragons", "lobby_getAdventureMapStats") => AccessLevel::Public,
            ("dragons", "lobby_getApplicationSettings") => AccessLevel::Public,
            ("dragons", "lobby_craftBingoRecipe") => AccessLevel::Protected,
            ("dragons", "lobby_getBingoData") => AccessLevel::Public,
            ("dragons", "lobby_getUsersCards") => AccessLevel::Public,
            ("dragons", "lobby_upgradeCard") => AccessLevel::Protected,
            ("dragons", "lobby_getClientVersion") => AccessLevel::Public,
            ("dragons", "lobby_getConfigById") => AccessLevel::Public,
            ("dragons", "lobby_updateConfig") => AccessLevel::Public,
            ("dragons", "lobby_buyCat") => AccessLevel::Protected,
            ("dragons", "lobby_buyScience") => AccessLevel::Protected,
            ("dragons", "lobby_buySkill") => AccessLevel::Protected,
            ("dragons", "lobby_buyUsualLootbox") => AccessLevel::Protected,
            ("dragons", "lobby_getBingoJackpotPool") => AccessLevel::Public,
            ("dragons", "lobby_getBingoJackpotWinnersInfo") => AccessLevel::Public,
            ("dragons", "lobby_getLootboxJackpotWinnersInfo") => AccessLevel::Public,
            ("dragons", "lobby_getLootboxJackpotPool") => AccessLevel::Public,
            ("dragons", "lobby_getPreviousTournamentStats") => AccessLevel::Public,
            ("dragons", "lobby_getSoftLeaderboardStats") => AccessLevel::Public,
            ("dragons", "lobby_getTournamentLeaderboardStats") => AccessLevel::Public,
            ("dragons", "lobby_getLootboxesInfo") => AccessLevel::Public,
            ("dragons", "lobby_getUsualLootboxTime") => AccessLevel::Public,
            ("dragons", "lobby_openLootbox") => AccessLevel::Protected,
            ("dragons", "lobby_getAvailableMembershipAmount") => AccessLevel::Public,
            ("dragons", "lobby_getOffersAndBonuses") => AccessLevel::Public,
            ("dragons", "lobby_refuseOffer") => AccessLevel::Public,
            ("dragons", "lobby_saveTutorialProgress") => AccessLevel::Public,
            ("dragons", "lobby_saveUsersProgress") => AccessLevel::Public,
            ("dragons", "lobby_getUsersReferrals") => AccessLevel::Protected,
            ("dragons", "lobby_setReferrer") => AccessLevel::Public,
            ("dragons", "lobby_getServerTime") => AccessLevel::Public,
            ("dragons", "lobby_keepAlive") => AccessLevel::Public,
            ("dragons", "lobby_getSpinRewards") => AccessLevel::Public,
            ("dragons", "lobby_useSpin") => AccessLevel::Public,
            ("dragons", "lobby_completeTask") => AccessLevel::Protected,
            ("dragons", "lobby_getCurrentTasks") => AccessLevel::Public,
            ("dragons", "lobby_getUsersCompletedTasks") => AccessLevel::Public,
            ("dragons", "lobby_userValidation") => AccessLevel::Public,
            ("dragons", "lobby_getBadTransactions") => AccessLevel::Public,
            ("dragons", "lobby_getUserIdByTransactionId") => AccessLevel::Public,
            ("dragons", "lobby_getUsersInappPurchases") => AccessLevel::Public,
            ("dragons", "lobby_resetDeviceId") => AccessLevel::Protected,
            ("dragons", "lobby_unlinkAccount") => AccessLevel::Protected,
            ("dragons", "lobby_getProfile") => AccessLevel::Public,
            ("dragons", "lobby_getSoulPrice") => AccessLevel::Public,
            ("dragons", "lobby_purchaseValidation") => AccessLevel::Public,
            ("dragons", "lobby_retrieveUsersProgress") => AccessLevel::Public,
            ("dragons", "lobby_shareScore") => AccessLevel::Public,
            ("dragons", "lobby_setUserAppMetricaDeviceId") => AccessLevel::Public,
            ("dragons", "lobby_getJackpotWinnersInfo") => AccessLevel::Public,
            ("dragons", "lobby_getProfileInfo") => AccessLevel::Public,
            ("dragons", "lobby_applicationInitialization") => AccessLevel::Public,
            ("email", "email_findAll") => AccessLevel::Protected,
            ("email", "email_createCronJob") => AccessLevel::Protected,
            ("email", "email_createAndSend") => AccessLevel::Protected,
            ("email", "email_sendNotSentLetters") => AccessLevel::Protected,
            ("email", "email_sendInProcessLetters") => AccessLevel::Protected,
            ("gameBalancer", "balancerEndpoints_addBots") => AccessLevel::Private,
            ("gameBalancer", "coefficientEndpoints_get") => AccessLevel::Private,
            ("gameBalancer", "coefficientEndpoints_update") => AccessLevel::Private,
            ("gameBalancer", "gameBalancer_addPlayerInSearch") => AccessLevel::Protected,
            ("gameBalancer", "gameBalancer_disconnectPlayer") => AccessLevel::Protected,
            ("gameBalancer", "gameBalancer_deletePlayers") => AccessLevel::Protected,
            ("gameBalancer", "gameBalancer_stopSearching") => AccessLevel::Protected,
            ("gameBalancer", "gameBalancer_gameStartedNotification") => AccessLevel::Protected,
            ("gameBalancer", "gameBalancer_checkInNotification") => AccessLevel::Protected,
            ("google-play", "purchase_validateSubscription") => AccessLevel::Internal,
            ("images", "image_uploadImage") => AccessLevel::Private,
            ("js-tests", "dragocatsBattleProcessEndpoints_startTest") => AccessLevel::Private,
            ("js-tests", "dragocatsBattleProcessEndpoints_stopTest") => AccessLevel::Private,
            ("js-tests", "dragocatsBattleProcessEndpoints_getTestState") => AccessLevel::Private,
            ("listeria-storage", "heroesEndpoints_getById") => AccessLevel::Internal,
            ("listeria-storage", "heroesEndpoints_levelUp") => AccessLevel::Protected,
            ("listeria-storage", "heroesEndpoints_giveFreeExperienceToHero") => {
                AccessLevel::Protected
            }
            ("listeria-storage", "heroesEndpoints_getHeroesListByUserId") => AccessLevel::Protected,
            ("listeria-storage", "heroesEndpoints_statsUpdatedNotification") => {
                AccessLevel::Protected
            }
            ("listeria-storage", "heroesEndpoints_heroAddedNotification") => AccessLevel::Protected,
            ("listeria-storage", "inventoryEndpoints_getById") => AccessLevel::Internal,
            ("listeria-storage", "inventoryEndpoints_getEquipmentScrollsCount") => {
                AccessLevel::Protected
            }
            ("listeria-storage", "inventoryEndpoints_getHeroScrollsCount") => {
                AccessLevel::Protected
            }
            ("listeria-storage", "inventoryEndpoints_getLootboxesList") => AccessLevel::Protected,
            ("listeria-storage", "inventoryEndpoints_openLootbox") => AccessLevel::Protected,
            ("listeria-storage", "inventoryEndpoints_addedUnequippableItemNotification") => {
                AccessLevel::Protected
            }
            ("listeria-storage", "inventoryEndpoints_getItemsList") => AccessLevel::Protected,
            ("listeria-storage", "inventoryEndpoints_levelUp") => AccessLevel::Protected,
            ("lobby", "equipmentEndpoints_equipItem") => AccessLevel::Protected,
            ("lobby", "equipmentEndpoints_unequipItem") => AccessLevel::Protected,
            ("lobby", "equipmentEndpoints_getItemByItemIdAndUserId") => AccessLevel::Protected,
            ("lobby", "equipmentEndpoints_updatedItemNotification") => AccessLevel::Protected,
            ("lobby", "equipmentEndpoints_addedItemNotification") => AccessLevel::Protected,
            ("lobby", "equipmentEndpoints_unEquipItemFromAllHeroes") => AccessLevel::Internal,
            ("lobby", "heroesEndpoints_getHeroDTOAttributes") => AccessLevel::Internal,
            ("lobby", "heroesEndpoints_getEquippedItems") => AccessLevel::Internal,
            ("lobby", "heroesEndpoints_equipHero") => AccessLevel::Protected,
            ("lobby", "heroesEndpoints_getHeroesList") => AccessLevel::Protected,
            ("lobby", "heroesEndpoints_getActiveHeroId") => AccessLevel::Internal,
            ("lobby", "heroesEndpoints_heroAddedNotification") => AccessLevel::Protected,
            ("lobby", "lobby_startGame") => AccessLevel::Protected,
            ("lobby", "regions_get") => AccessLevel::Protected,
            ("lobby", "regions_set") => AccessLevel::Protected,
            ("lobby", "regions_getList") => AccessLevel::Private,
            ("lobby", "settings_get") => AccessLevel::Protected,
            ("lobby", "settings_set") => AccessLevel::Protected,
            ("lobby", "statsEndpoints_getLevelUpOptionsByPersonalType") => AccessLevel::Internal,
            ("lobby", "statsEndpoints_getStatsOptionsByPersonalType") => AccessLevel::Internal,
            ("lobby", "statsOptionsEndpoints_createOrUpdate") => AccessLevel::Private,
            ("lobby", "statsOptionsEndpoints_getByPersonalType") => AccessLevel::Private,
            ("lobby", "statsOptionsEndpoints_getPersonalTypesList") => AccessLevel::Private,
            ("lobby", "user_getUserData") => AccessLevel::Protected,
            ("lobby", "user_updateUsername") => AccessLevel::Protected,
            ("lobby", "user_getLeaderBoard") => AccessLevel::Protected,
            ("lobby", "user_updateProfileImage") => AccessLevel::Protected,
            ("lootboxes", "lootboxes_openLootbox") => AccessLevel::Protected,
            ("lootboxes", "lootboxes_getLootboxesInfo") => AccessLevel::Public,
            ("lootboxes", "lootboxes_getUsualLootboxTime") => AccessLevel::Public,
            ("market", "items_getPrice") => AccessLevel::Internal,
            ("market", "items_getLootboxesList") => AccessLevel::Public,
            ("market-place", "marketEndpoints_getByFilter") => AccessLevel::Public,
            ("market-place", "marketEndpoints_getSimilar") => AccessLevel::Internal,
            ("market-place", "marketPlace_buyItem") => AccessLevel::Internal,
            ("market-place", "marketPlace_unlockItem") => AccessLevel::Internal,
            ("market-place", "marketPlace_getItemById") => AccessLevel::Internal,
            ("market-place", "marketPlace_getBoughtItems") => AccessLevel::Protected,
            ("market-place", "marketPlace_getItemsByFilter") => AccessLevel::Public,
            ("market-place", "marketPlace_lockItem") => AccessLevel::Internal,
            ("market-place", "marketPlace_cancelSale") => AccessLevel::Protected,
            ("market-place", "marketPlace_getCanceledItems") => AccessLevel::Protected,
            ("market-place", "marketPlace_addToMarketPlace") => AccessLevel::Protected,
            ("near-adapter", "contract_isEnoughBalanceOnWithdrawWallet") => AccessLevel::Internal,
            ("near-adapter", "contract_callWithdraw") => AccessLevel::Internal,
            ("near-adapter", "contract_callTransfer") => AccessLevel::Internal,
            ("near-adapter", "contract_callMintNft") => AccessLevel::Protected,
            ("near-adapter", "contract_callBurnNft") => AccessLevel::Protected,
            ("near-adapter", "contract_callNftLock") => AccessLevel::Protected,
            ("near-adapter", "contract_callNftUnlock") => AccessLevel::Protected,
            ("near-adapter", "contract_callNftUnlockAndTransfer") => AccessLevel::Protected,
            ("near-adapter", "wallet_getMy") => AccessLevel::Protected,
            ("near-adapter", "wallet_getMyNft") => AccessLevel::Protected,
            ("near-adapter", "wallet_getUserByAccountId") => AccessLevel::Private,
            ("notifications", "notifications_createNew") => AccessLevel::Protected,
            ("orchestrator", "balance_increaseUserBalance") => AccessLevel::Internal,
            ("orchestrator", "balance_userBlockchainDeposit") => AccessLevel::Internal,
            ("orchestrator", "balance_decreaseUserBalance") => AccessLevel::Internal,
            ("orchestrator", "battlePass_purchasePremium") => AccessLevel::Internal,
            ("orchestrator", "battlePass_purchaseExperience") => AccessLevel::Internal,
            ("orchestrator", "battlePass_generateProduct") => AccessLevel::Internal,
            ("orchestrator", "bingo_reward") => AccessLevel::Internal,
            ("orchestrator", "lootbox_open") => AccessLevel::Internal,
            ("orchestrator", "lootbox_purchase") => AccessLevel::Protected,
            ("orchestrator", "marketPlace_purchaseItem") => AccessLevel::Internal,
            ("orchestrator", "product_burn") => AccessLevel::Internal,
            ("orchestrator", "purchase_purchaseProduct") => AccessLevel::Protected,
            ("orchestrator", "wallet_getMy") => AccessLevel::Protected,
            ("orchestrator", "wallet_getByUserId") => AccessLevel::Private,
            ("product-factory", "generator_generate") => AccessLevel::Protected,
            ("product-factory", "lootbox_open") => AccessLevel::Protected,
            ("product-factory", "products_getInfoByProductId") => AccessLevel::Internal,
            ("productFactory", "lootboxType_add") => AccessLevel::Private,
            ("productFactory", "lootboxType_getAll") => AccessLevel::Private,
            ("productFactory", "lootboxType_updateByLootboxId") => AccessLevel::Private,
            ("productFactory", "lootboxType_getByLootboxId") => AccessLevel::Protected,
            ("productFactory", "product_markHeroAsNFT") => AccessLevel::Internal,
            ("productFactory", "product_addLootbox") => AccessLevel::Internal,
            ("productFactory", "product_lootboxOpenedNotification") => AccessLevel::Protected,
            ("productFactory", "productType_add") => AccessLevel::Private,
            ("productFactory", "productType_isExist") => AccessLevel::Internal,
            ("productFactory", "productType_update") => AccessLevel::Private,
            ("productFactory", "productType_delete") => AccessLevel::Private,
            ("productFactory", "productType_getAll") => AccessLevel::Private,
            ("productFactory", "productType_get") => AccessLevel::Private,
            ("productFactory", "productType_getAllByType") => AccessLevel::Private,
            ("productFactory", "productType_getAllByRarity") => AccessLevel::Private,
            ("productFactory", "productType_getHash") => AccessLevel::Internal,
            ("productFactory", "productType_getAttributeListByPersonalType") => {
                AccessLevel::Protected
            }
            ("promo", "codes_useCode") => AccessLevel::Protected,
            ("promo", "codes_createCode") => AccessLevel::Private,
            ("promo", "codes_getDataByCodeStrict") => AccessLevel::Private,
            ("promo", "codes_getListStrict") => AccessLevel::Private,
            ("promo", "codes_deleteCode") => AccessLevel::Private,
            ("purchase", "balance_increaseUserBalance") => AccessLevel::Internal,
            ("purchase", "balance_decreaseUserBalance") => AccessLevel::Internal,
            ("purchase", "balance_userBalanceChangedNotification") => AccessLevel::Protected,
            ("purchase", "product_purchaseLootbox") => AccessLevel::Protected,
            ("purchase", "product_getHeroList") => AccessLevel::Protected,
            ("purchase", "product_getEquipmentList") => AccessLevel::Protected,
            ("purchase", "wallet_getByUserId") => AccessLevel::Protected,
            ("purchase", "wallet_getWalletByUserId") => AccessLevel::Protected,
            ("realis", "walletManager_getMyAddress") => AccessLevel::Protected,
            ("referral", "link_get") => AccessLevel::Protected,
            ("referral", "link_getMy") => AccessLevel::Protected,
            ("referral", "link_getAll") => AccessLevel::Private,
            ("referral", "link_getAllMy") => AccessLevel::Protected,
            ("referral", "referral_getReferralInfoList") => AccessLevel::Protected,
            ("referral", "referral_addReferral") => AccessLevel::Private,
            ("referral", "referral_setPotentialReferral") => AccessLevel::Public,
            ("referral", "referral_getReferrals") => AccessLevel::Private,
            ("referral", "referral_getUserData") => AccessLevel::Internal,
            ("referral", "reward_addReferralExpense") => AccessLevel::Internal,
            ("referral", "reward_makeRewardRequest") => AccessLevel::Protected,
            ("referral", "reward_getAvailableReward") => AccessLevel::Private,
            ("refund", "balances_getAllMy") => AccessLevel::Private,
            ("refund", "balances_getAll") => AccessLevel::Protected,
            ("refund", "balances_delete") => AccessLevel::Protected,
            ("refund", "balances_getAllUnavailable") => AccessLevel::Protected,
            ("refund", "balances_add") => AccessLevel::Protected,
            ("refund", "balances_getAllLockedFunds") => AccessLevel::Internal,
            ("refund", "items_getAllMy") => AccessLevel::Private,
            ("refund", "items_getAll") => AccessLevel::Protected,
            ("refund", "items_delete") => AccessLevel::Protected,
            ("refund", "items_isAvailable") => AccessLevel::Protected,
            ("refund", "items_getAllUnavailable") => AccessLevel::Protected,
            ("refund", "items_add") => AccessLevel::Protected,
            ("soul-adapter", "wallet_getUserIdByAddress") => AccessLevel::Internal,
            ("soul-adapter", "wallet_getMyWallet") => AccessLevel::Protected,
            ("soul-adapter", "wallet_processTransactionByHash") => AccessLevel::Private,
            ("soul-adapter", "wallet_getBalanceByAddress") => AccessLevel::Private,
            ("status", "config_add") => AccessLevel::Private,
            ("status", "config_delete") => AccessLevel::Private,
            ("status", "config_disable") => AccessLevel::Private,
            ("status", "config_update") => AccessLevel::Private,
            ("status", "config_getAll") => AccessLevel::Private,
            ("status", "config_getMembershipInfo") => AccessLevel::Private,
            ("status", "config_updatePriorityIndex") => AccessLevel::Private,
            ("status", "config_getList") => AccessLevel::Private,
            ("status", "config_getAllForPurchase") => AccessLevel::Public,
            ("status", "membership_update") => AccessLevel::Internal,
            ("status", "membership_addAwardMembership") => AccessLevel::Private,
            ("status", "membership_getInfo") => AccessLevel::Internal,
            ("status", "membership_changeMembershipGame") => AccessLevel::Private,
            ("status", "membership_disable") => AccessLevel::Private,
            ("status", "membership_getAllActive") => AccessLevel::Protected,
            ("status", "membership_getAllMyActive") => AccessLevel::Protected,
            ("status", "membership_purchaseMembershipFromSite") => AccessLevel::Protected,
            ("status", "membership_getUserByGPA") => AccessLevel::Private,
            ("status", "membership_checkUserHasActiveStatus") => AccessLevel::Protected,
            ("task", "task_completeTask") => AccessLevel::Protected,
            ("task", "task_getCurrentTasks") => AccessLevel::Public,
            ("task", "task_getUsersCompletedTasks") => AccessLevel::Public,
            ("transactions", "balance_getBalanceByUserId") => AccessLevel::Protected,
            ("transactions", "balance_getBalancesByUserId") => AccessLevel::Protected,
            ("transactions", "balance_getBalancesByUserIdAsArray") => AccessLevel::Protected,
            ("transactions", "balance_increaseBalanceByUserId") => AccessLevel::Internal,
            ("transactions", "balance_decreaseBalanceByUserId") => AccessLevel::Internal,
            ("transactions", "balance_getBalancesInUsd") => AccessLevel::Protected,
            ("transactions", "balance_convert") => AccessLevel::Protected,
            ("transactions", "balance_getMyBalancesWithRounding") => AccessLevel::Protected,
            ("transactions", "balance_getAll") => AccessLevel::Private,
            ("transactions", "balance_getAllMy") => AccessLevel::Protected,
            ("transactions", "balance_getMyNumOfTransactions") => AccessLevel::Protected,
            ("transactions", "balance_getAllCreditTransactionList") => AccessLevel::Protected,
            ("transactions", "balance_getUserBalances") => AccessLevel::Protected,
            ("transactions", "balance_getWithFilter") => AccessLevel::Protected,
            ("transactions", "balance_getNumOfTransactions") => AccessLevel::Private,
            ("transactions", "balance_updateTransactionHashAndBlockId") => AccessLevel::Internal,
            ("transactions", "balance_updateTransactionDataFromBlockchain") => {
                AccessLevel::Internal
            }
            ("transactions", "balance_getListWithPagination") => AccessLevel::Protected,
            ("transactions", "balance_deleteBalanceByUserId") => AccessLevel::Internal,
            ("transactions", "balance_getAllLisSum") => AccessLevel::Public,
            ("transactions", "balance_adminIncreaseBalance") => AccessLevel::Private,
            ("transactions", "balance_adminDecreaseBalance") => AccessLevel::Private,
            ("transactions", "balance_getTestnetLis") => AccessLevel::Protected,
            ("transactions", "balance_getNumWithFilter") => AccessLevel::Private,
            ("transactions", "balance_checkTransactionForCurrency") => AccessLevel::Internal,
            ("transactions", "balance_checkMyTransactionForCurrency") => AccessLevel::Protected,
            ("transactions", "registryProduct_addProduct") => AccessLevel::Internal,
            ("transactions", "registryProduct_addProductHash") => AccessLevel::Internal,
            ("transactions", "registryProduct_getPersonalTypesByUser") => AccessLevel::Internal,
            ("transactions", "registryProduct_deleteProductByUserId") => AccessLevel::Internal,
            ("transactions", "registryProduct_burnProduct") => AccessLevel::Internal,
            ("transactions", "registryProduct_updateProductOwner") => AccessLevel::Internal,
            ("user", "profile_changeNickname") => AccessLevel::Protected,
            ("user", "profile_getProfile") => AccessLevel::Private,
            ("user", "profile_getMyProfile") => AccessLevel::Protected,
            ("user", "profile_getAllProfiles") => AccessLevel::Private,
            ("user", "profile_unsetNotice") => AccessLevel::Private,
            ("user", "profile_setSuspicious") => AccessLevel::Private,
            ("user", "profile_unsetSuspicious") => AccessLevel::Private,
            ("user", "profile_banProfile") => AccessLevel::Private,
            ("user", "profile_unBan") => AccessLevel::Private,
            ("user", "profile_setMailingSubscriptionStatus") => AccessLevel::Protected,
            ("user", "profile_unsubscribeFromNewsletter") => AccessLevel::Public,
            ("user", "profile_getAllUsersSubscribedToMailing") => AccessLevel::Public,
            ("user", "profile_getBanInfo") => AccessLevel::Private,
            ("user", "profile_getMyBanInfo") => AccessLevel::Protected,
            ("user", "profile_getNicknameChangePrice") => AccessLevel::Protected,
            ("user", "profile_getUserByParams") => AccessLevel::Protected,
            ("user", "profile_getMyNickname") => AccessLevel::Protected,
            ("user", "profile_getUserNickname") => AccessLevel::Internal,
            ("user", "profile_getNicknamesByUserIds") => AccessLevel::Internal,
            ("user", "profile_deleteUser") => AccessLevel::Private,
            ("user", "profile_setNotice") => AccessLevel::Private,
            ("user", "profile_changeEmail") => AccessLevel::Internal,
            ("user", "profile_isEmailExists") => AccessLevel::Internal,
            ("user", "profile_getUserIdByEmail") => AccessLevel::Internal,
            ("user", "profile_deleteUserRecord") => AccessLevel::Internal,
            ("user", "profile_getNum") => AccessLevel::Public,
            ("user", "profile_getMyProfileForBytes") => AccessLevel::Protected,
            ("user", "profile_getTestData") => AccessLevel::Protected,
            ("user", "profile_getCountActives") => AccessLevel::Public,
            ("user", "status_delete") => AccessLevel::Private,
            ("user", "status_get") => AccessLevel::Private,
            ("withdraw", "approval_getAllMyTransactions") => AccessLevel::Protected,
            ("withdraw", "approval_listUnapproved") => AccessLevel::Private,
            ("withdraw", "approval_approve") => AccessLevel::Private,
            ("withdraw", "approval_deny") => AccessLevel::Private,
            ("withdraw", "attempt_tryNew") => AccessLevel::Protected,
            _ => None,
        }
    }
    pub fn validate_params(
        agent: &str,
        method: &str,
        json: &Value,
    ) -> Result<(), BaseError<Value>> {
        let schema = Self::get_params_schema(agent, method)
            .ok_or_else(|| BaseError::new("Unknown method", Common::InternalServerError))?;
        Self::validate(&schema, json)
    }
    pub fn validate_returns(
        agent: &str,
        method: &str,
        json: &Value,
    ) -> Result<(), BaseError<Value>> {
        let schema = Self::get_returns_schema(agent, method)
            .ok_or_else(|| BaseError::new("Unknown method", Common::InternalServerError))?;
        Self::validate(&schema, json)
    }
    pub fn validate(schema: &Value, json: &Value) -> Result<(), BaseError<Value>> {
        JSONSchema::compile(schema)
            .map_err(|e| {
                BaseError::new(
                    format!("Invalid schema: {}", e),
                    Common::InternalServerError,
                )
            })?
            .validate(json)
            .map_err(|e| BaseError {
                msg: format!(
                    "Does not match pattern: {:?}",
                    e.map(|e| e.to_string()).collect::<Vec<_>>()
                ),
                error_type: Validation::DoesNotMatchPattern.into(),
                data: Some(json ! ({ "schema" : schema , "json" : json , })),
                ..Default::default()
            })
    }
    pub fn is_valid_params(agent: &str, method: &str, json: &Value) -> Result<bool, BaseError<()>> {
        let schema = Self::get_params_schema(agent, method)
            .ok_or_else(|| BaseError::new("Unknown method", Common::InternalServerError))?;
        Self::is_valid(&schema, json)
    }
    pub fn is_valid_returns(
        agent: &str,
        method: &str,
        json: &Value,
    ) -> Result<bool, BaseError<()>> {
        let schema = Self::get_returns_schema(agent, method)
            .ok_or_else(|| BaseError::new("Unknown method", Common::InternalServerError))?;
        Self::is_valid(&schema, json)
    }
    pub fn is_valid(schema: &Value, json: &Value) -> Result<bool, BaseError<()>> {
        JSONSchema::compile(schema)
            .map(|schema| schema.is_valid(json))
            .map_err(|e| {
                BaseError::new(
                    format!("Invalid schema: {}", e),
                    Common::InternalServerError,
                )
            })
    }
}
