import dns from 'node:dns';

import { describe, test } from 'node:test';
import assert from 'node:assert';
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

            assert.strictEqual(status, 401);
        });

        test('should successfully query when Bearer prefix is provided', async () => {
            const {status} = await fetch('http://localhost:4000?query={__typename}', {
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': 'Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c'
                }
            });

            assert.strictEqual(status, 200);
        });
    });
});