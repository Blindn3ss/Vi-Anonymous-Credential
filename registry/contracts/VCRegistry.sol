// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract VCRegistry {
    event CredentialIssued(bytes32 indexed credId, address indexed to);
    event CredentialRevoked(bytes32 indexed credId);

    mapping(bytes32 => bool) public issued;
    mapping(bytes32 => bool) public revoked;

    function issueCredential(bytes32 credId, address to) public {
        require(!issued[credId], "Already issued");
        issued[credId] = true;
        emit CredentialIssued(credId, to);
    }

    function revokeCredential(bytes32 credId) public {
        require(issued[credId], "Not issued");
        revoked[credId] = true;
        emit CredentialRevoked(credId);
    }

    function isRevoked(bytes32 credId) public view returns (bool) {
        return revoked[credId];
    }
}
