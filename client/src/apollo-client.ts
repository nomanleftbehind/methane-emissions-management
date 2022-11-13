import {
	ApolloClient,
	InMemoryCache,
	HttpLink, split
} from '@apollo/client/core';
import { GraphQLWsLink } from "@apollo/client/link/subscriptions";
import { createClient } from "graphql-ws";
import { getMainDefinition } from '@apollo/client/utilities';

const cache = new InMemoryCache({
	addTypename: true,
});

const wsLink = typeof window !== 'undefined' ? new GraphQLWsLink(createClient({
	url: 'ws://127.0.0.1:8081/graphql',
})) : null;

const httpLink = new HttpLink({
	uri: 'http://127.0.0.1:8081/graphql',
	credentials: 'include',
});

const splitLink = typeof window !== 'undefined' ? split(
	({ query }) => {
		const definition = getMainDefinition(query);
		return (
			definition.kind === 'OperationDefinition' &&
			definition.operation === 'subscription'
		);
	},
	wsLink!,
	httpLink,
) : httpLink;

export default new ApolloClient({
	cache,
	link: splitLink,
	connectToDevTools: true,
});