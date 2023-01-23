import dns from 'node:dns';
import { describe, test } from 'node:test';
import assert from 'node:assert';

import jwt from 'jsonwebtoken';

dns.setDefaultResultOrder('ipv4first');

describe('e2e', () => {
    describe('when auth header is not provided', () => {
        test('should successfully query server', async () => {
            const {status} = await fetch('http://localhost:4000?query={__typename}', {
                headers: {
                    'Content-Type': 'application/json'
                }
            });

            assert.strictEqual(status, 200);
        });
    });

    describe('when auth header is provided', () => {
        test('should return unauthorized when Bearer prefix is not provided', async () => {
            const {status} = await fetch('http://localhost:4000?query={__typename}', {
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c'
                }
            });
            console.log('testing status', status);
            assert.strictEqual(status, 401);
        });

        test('should return unauthorized when iss is not provided', async () => {
            const token = jwt.sign({
                cid: 'testCid',
                uid: 'testUid',
            }, 'testValue');

            const {status} = await fetch('http://localhost:4000?query={__typename}', {
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                }
            });

            assert.strictEqual(status, 401);
        });

        test('should return unauthorized when iss is wrong', async () => {
            const token = jwt.sign({
                cid: 'testCid',
                uid: 'testUid',
                iss: 'https://example3.oktapreview.com/oauth2/123456abcde'
            }, 'testValue');

            const {status} = await fetch('http://localhost:4000?query={__typename}', {
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                }
            });

            assert.strictEqual(status, 401);
        });

        test('should successfully query when proper JWT is provided', async () => {
            const token = jwt.sign({
                cid: 'testCid',
                uid: 'testUid',
                iss: 'https://example1.oktapreview.com/oauth2/123456abcd'
            }, 'testValue');

            const {status} = await fetch('http://localhost:4000?query={__typename}', {
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                }
            });

            assert.strictEqual(status, 200);
        });
    });
});