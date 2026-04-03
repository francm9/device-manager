interface BrokerInfoRepository {
  get(): Promise<BrokerInfo>
}
